use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_hcl::FromHcl;
mod nu_hcl;

fn main() {
    serve_plugin(&mut FromHcl::new(), MsgPackSerializer)
}
