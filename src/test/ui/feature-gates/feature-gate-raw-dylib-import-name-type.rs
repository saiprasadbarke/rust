// only-windows
// only-x86
#[link(name = "foo", kind = "raw-dylib", import_name_type = "decorated")]
//~^ ERROR link kind `raw-dylib` is unstable
//~| ERROR import name type is unstable
extern "C" {}

fn main() {}
