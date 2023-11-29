use nu_plugin::{EvaluatedCall, LabeledError, Plugin};
use nu_protocol::{record, Category, PluginExample, PluginSignature, Span, Type, Value};
use serde_json::Value as SerdeJsonValue;

pub struct FromHcl;

impl FromHcl {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn examples() -> Vec<PluginExample> {
    let span = Span::test_data();
    let vec = vec![PluginExample {
        description: "Convert .hcl data into record".into(),
        example: "'provider \"aws\" {
  region = \"us-east-1\"
}
resource \"aws_instance\" \"web\" {
  ami           = \"ami-a1b2c3d4\"
  instance_type = \"t2.micro\"
}' | from hcl"
            .into(),
        result: Some(Value::record(
            record! {
                    "provider".to_string()=>Value::record(
                        record!{
                            "aws".to_string() => Value::record
                            (    record!{
                                "region".to_string()=>Value::test_string("us-east-1")
                            },span
                            )
                        }
                    ,span
                    ),

                    "resource".to_string()=> Value::record(
                        record!{"aws_instance".to_string()=>
                        Value::record (
                            record!{
                                "web".to_string()=>
                                Value::record(
                                    record! {
                                        "ami".to_string()=>Value::test_string("ami-a1b2c3d4"),
                                        "instance_type".to_string()=>Value::test_string("t2.micro"),
                                    },
                                    span,
                                )
                            },
                            span
                        )}
                        ,
                         span,
            )
                },
            span,
        )),
    }];
    vec
}

pub fn convert_sjson_to_value(value: &SerdeJsonValue, span: Span) -> Value {
    match value {
        SerdeJsonValue::Array(array) => {
            let v: Vec<Value> = array
                .iter()
                .map(|x| convert_sjson_to_value(x, span))
                .collect();

            Value::list(v, span)
        }
        SerdeJsonValue::Bool(b) => Value::bool(*b, span),
        SerdeJsonValue::Number(f) => {
            if f.is_f64() {
                Value::float(f.as_f64().unwrap(), span)
            } else {
                Value::int(f.as_i64().unwrap(), span)
            }
        }
        SerdeJsonValue::Null => Value::nothing(span),
        SerdeJsonValue::Object(k) => {
            let mut rec = record!();
            for item in k {
                rec.push(item.0.clone(), convert_sjson_to_value(item.1, span));
            }

            Value::record(rec, span)
        }
        SerdeJsonValue::String(s) => Value::string(s.clone(), span),
    }
}

impl Plugin for FromHcl {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("from hcl")
            .input_output_types(vec![(Type::String, Type::Record(vec![]))])
            .usage("Parse text as .hcl and create a record")
            .plugin_examples(examples())
            .category(Category::Formats)]
    }

    fn run(
        &mut self,
        _name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = call.head;
        let input_string = input.as_string()?;

        let parse_result: SerdeJsonValue =
            hcl::from_str(&input_string).map_err(|e| LabeledError {
                label: "Error parsing hcl".into(),
                msg: e.to_string(),
                span: Some(span),
            })?;

        Ok(convert_sjson_to_value(&parse_result, span))
    }
}
