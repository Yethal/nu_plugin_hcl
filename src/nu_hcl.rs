use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{
    record, Category, ErrorLabel, Example, LabeledError, Signature, Span, Type, Value,
};
use serde_json::Value as SerdeJsonValue;

pub struct HclPlugin;

impl Plugin for HclPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn commands(&self) -> Vec<Box<dyn nu_plugin::PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromHcl), Box::new(FromTf)]
    }
}

struct FromHcl;

impl SimplePluginCommand for FromHcl {
    type Plugin = HclPlugin;

    fn name(&self) -> &str {
        "from hcl"
    }

    fn signature(&self) -> nu_protocol::Signature {
        signature(PluginCommand::name(self))
    }

    fn usage(&self) -> &str {
        "Parse text as .hcl and create a record"
    }

    fn examples(&self) -> Vec<Example> {
        examples("Convert .hcl data into record")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        run(call, input)
    }
}

pub struct FromTf;

impl SimplePluginCommand for FromTf {
    type Plugin = HclPlugin;

    fn name(&self) -> &str {
        "from tf"
    }

    fn signature(&self) -> nu_protocol::Signature {
        signature(PluginCommand::name(self))
    }

    fn usage(&self) -> &str {
        "Parse text as .tf and create a record"
    }

    fn examples(&self) -> Vec<Example> {
        examples("Convert .tf data into record")
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        run(call, input)
    }
}

fn signature(name: &str) -> nu_protocol::Signature {
    Signature::build(name)
        .input_output_type(Type::String, Type::Record(Box::new([])))
        .category(Category::Formats)
}

fn examples(description: &str) -> Vec<Example> {
    let span = Span::test_data();
    let vec = vec![Example {
        description,
        example: "'provider \"aws\" {
  region = \"us-east-1\"
}
resource \"aws_instance\" \"web\" {
  ami           = \"ami-a1b2c3d4\"
  instance_type = \"t2.micro\"
}' | from hcl",
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

fn run(call: &EvaluatedCall, input: &Value) -> Result<Value, LabeledError> {
    let span = call.head;
    let input_string = input.as_str()?;

    let parse_result: SerdeJsonValue = hcl::from_str(input_string).map_err(|e| LabeledError {
        labels: vec![ErrorLabel {
            text: "Error parsing hcl".into(),
            span,
        }],
        msg: e.to_string(),
        code: None,
        url: None,
        help: None,
        inner: Vec::default(),
    })?;

    Ok(convert_sjson_to_value(&parse_result, span))
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
