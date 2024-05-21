[package]
name = "abc"
version = "0.0.1"
edition = "2021"

[dependencies]
subcrate = { path = "subcrate/" }

mkdir -p src subcrate/src

[package]
name = "subcrate"
version = "0.0.1"
edition = "2021"

#![feature(generic_const_exprs)]
#![feature(adt_const_params)]

#[derive(PartialEq, Eq)]
pub struct Const {}
impl Const {
    pub const fn func(self) -> usize { 1 }
}

pub struct Foo<const C: Const>
where
    [(); C.func()]: Sized
{}

pub trait Bar {
    type Associated;
    fn associated() -> Self::Associated;
}

impl<const C: Const> Bar for Foo<C>
where
    [(); C.func()]: Sized
{
    type Associated = [(); C.func()];
    fn associated() -> Self::Associated {
        [(); C.func()]
    }
}

use subcrate::*;

fn main() {
    <Foo::<{Const{}}> as Bar>::associated();
}
