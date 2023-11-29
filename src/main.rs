use nu_hcl::FromHcl;
use nu_plugin::{serve_plugin, MsgPackSerializer};
mod nu_hcl;

fn main() {
    serve_plugin(&mut FromHcl::new(), MsgPackSerializer)
}
