// ignore-tidy-linelength
// only-windows
// only-x86
#![feature(raw_dylib)]
//~^ WARN the feature `raw_dylib` is incomplete

#[link(name = "foo", kind = "raw-dylib", import_name_type = "decorated", import_name_type = "decorated")]
//~^ ERROR multiple `import_name_type` arguments in a single `#[link]` attribute
extern "C" { }

fn main() {}
