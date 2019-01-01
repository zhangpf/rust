#![feature(
    foo_bar_baz,
    foo(bar),
    foo = "baz"
)]
//~^^^ ERROR: malformed feature
//~^^^ ERROR: malformed feature

#![feature] //~ ERROR: attribute must be of the form `#[feature(...)]`
#![feature = "foo"] //~ ERROR: attribute must be of the form `#[feature(...)]`

#![feature(test_removed_feature)] //~ ERROR: feature has been removed

fn main() {}
