#![feature(custom_attribute)]

#[macro_use(x = 1usize)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1u8)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1u16)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1u32)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1u64)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1isize)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1i8)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1i16)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1i32)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1i64)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1.0f32)] //~ ERROR: suffixed literals are not allowed in attributes
#[macro_use(x = 1.0f64)] //~ ERROR: suffixed literals are not allowed in attributes
fn main() { }
