//@ aux-build:external-macro-src.rs

#![crate_name = "foo"]

#[macro_use]
extern crate external_macro_src;

// @has foo/index.html '//a[@href="../src/foo/external-macro-src.rs.html#3-12"]' 'source'

// @has foo/struct.Foo.html
// @has - '//a[@href="../src/foo/external-macro-src.rs.html#12"]' 'source'
make_foo!();
