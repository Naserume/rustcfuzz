#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub trait Foo {
    const SIZE: usize;
}

impl Foo for u64 {
    const SIZE: usize = 8;
}

pub struct Wrapper<T>
where
    T: Foo,
    [(); T::SIZE]:, // Remove this trait bound and everything will be ok
{
    pub t: T,
}

pub fn bar() -> Wrapper<impl Foo> {
    Wrapper { t: 10 }
}
