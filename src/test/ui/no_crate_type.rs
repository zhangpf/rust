// regression test for issue 11256
#![crate_type]  //~ ERROR attribute must be of the form `#[crate_type = "..."]`

fn main() {
    return
}
