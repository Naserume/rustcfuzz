#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

pub trait Bt {
    type C;
}

pub struct A;

impl Bt for A {
    type C = A;
}

pub fn oopsie<Bt>()
where
    for<'a, B> <B::C as Bt<'a>>::Bt: Bt, {}
