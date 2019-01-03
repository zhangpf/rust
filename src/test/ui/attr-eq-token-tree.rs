// compile-pass

#![feature(custom_attribute)]

#[my_attr = !] // OK under feature gate
fn main() {}
