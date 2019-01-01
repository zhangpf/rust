#[repr]
//~^ ERROR attribute must be of the form `#[repr(...)]`
//~| WARN `repr` attribute must have a hint
struct _A {}

#[repr = "B"]
//~^ ERROR attribute must be of the form `#[repr(...)]`
//~| WARN `repr` attribute isn't configurable with a literal
struct _B {}

#[repr = "C"]
//~^ ERROR attribute must be of the form `#[repr(...)]`
//~| WARN `repr` attribute isn't configurable with a literal
struct _C {}

#[repr(C)]
struct _D {}

fn main() {}
