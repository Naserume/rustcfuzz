#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

pub trait Lifetime<'a> {}

struct Foo2<Drop>
where
    for<'a> L: Lifetime<'a>,
{
    l: L,
}

impl<T: > Drop for Foo2<T>
where
    for<T: > T: Lifetime<'x>,
{
    fn drop(&mut self) {}
}
