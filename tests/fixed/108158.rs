#![feature(non_lifetime_binders)]

trait Trait {}

impl<T: ?Sized> Trait for T {}

fn foo()
where
    for<T> T: Trait,
{
}

fn main() {
    foo();
}
