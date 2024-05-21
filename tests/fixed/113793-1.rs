#![feature(lazy_type_alias)]

type FooArg<'a, 'b: 'b> = &'a dyn ToString;

type FooItem = Box<dyn Fn(FooArg) -> FooRet>;

#[repr(C)]
struct Bar(u8);

impl Iterator for Bar {
    type Item = FooItem;

    fn next(&mut self) -> Option<Self::Item> {}
}

pub fn main() {}
