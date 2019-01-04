#![deny = "foo"] //~ ERROR attribute must be of the form `#[deny(...)]`
#![allow(bar = "baz")] //~ ERROR malformed lint attribute

fn main() { }
