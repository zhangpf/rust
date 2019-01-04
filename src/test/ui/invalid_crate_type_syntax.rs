// regression test for issue 16974
#![crate_type(lib)]  //~ ERROR attribute must be of the form `#[crate_type = "..."]`

fn my_lib_fn() {}

fn main() {}
