use nu_hcl::HclPlugin;
use nu_plugin::{serve_plugin, MsgPackSerializer};
mod nu_hcl;

fn main() {
    serve_plugin(&HclPlugin, MsgPackSerializer)
}
