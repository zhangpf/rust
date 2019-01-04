// compile-flags: -Z parse-only

#[path*] //~ ERROR expected one of `(`, `::`, `=`, `[`, `]`, or `{`, found `*`
mod m {}
