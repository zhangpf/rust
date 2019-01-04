#[cfg_attr] //~ ERROR expected `(`, found `<eof>`
struct S1;

#[cfg_attr = ""] //~ ERROR expected `(`, found `=`
struct S2;

#[derive] //~ ERROR attribute must be of the form `#[derive(...)]`
struct S3;

#[derive = ""] //~ ERROR attribute must be of the form `#[derive(...)]`
struct S4;

fn main() {}
