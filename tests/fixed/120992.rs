#![feature(unnamed_fields)]
#[repr(C)]
struct D {
    _: Foo,
    _: union {
    }
}

pub fn main() {}
