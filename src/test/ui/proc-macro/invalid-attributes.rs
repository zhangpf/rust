// force-host
// no-prefer-dynamic

#![crate_type = "proc-macro"]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro = "test"] //~ ERROR attribute must be of the form `#[proc_macro]`
pub fn a(a: TokenStream) -> TokenStream { a }

#[proc_macro()] //~ ERROR attribute must be of the form `#[proc_macro]`
pub fn c(a: TokenStream) -> TokenStream { a }

#[proc_macro(x)] //~ ERROR attribute must be of the form `#[proc_macro]`
pub fn d(a: TokenStream) -> TokenStream { a }

#[proc_macro_attribute = "test"] //~ ERROR attribute must be of the form `#[proc_macro_attribute]`
pub fn e(_: TokenStream, a: TokenStream) -> TokenStream { a }

#[proc_macro_attribute()] //~ ERROR attribute must be of the form `#[proc_macro_attribute]`
pub fn g(_: TokenStream, a: TokenStream) -> TokenStream { a }

#[proc_macro_attribute(x)] //~ ERROR attribute must be of the form `#[proc_macro_attribute]`
pub fn h(_: TokenStream, a: TokenStream) -> TokenStream { a }
