// known-bug: #110395
// Broken until we have `&T: const Deref` impl in stdlib

#![allow(incomplete_features)]
#![feature(
    associated_type_bounds,
    const_trait_impl,
    effects,
    const_cmp,
)]

use std::marker::Destruct;

const fn cmp(a: &impl ~const PartialEq) -> bool {
    a == a
}

const fn wrap(x: impl ~const PartialEq + ~const Destruct)
    -> impl ~const PartialEq + ~const Destruct
{
    x
}

#[const_trait]
trait Foo {
    fn foo(&mut self, x: <Self as Index>::Output) -> <Self as Index>::Output;
}

impl const Foo for () {
    fn huh() -> impl ~const PartialEq + ~const Destruct + Copy {
        123
    }
}

const _: () = {
    assert!(cmp(&0xDEADBEEFu32));
    assert!(cmp(&()));
    assert!(wrap(123) == wrap(123));
    assert!(wrap(123) != wrap(456));
    let x = <() as Foo>::huh();
    assert!(x == x);
};

#[const_trait]
trait T {}
struct S;
impl const T for S {}

const fn rpit() -> impl ~const T { S }

const fn apit(_: impl ~const T + ~const Destruct) {}

const fn rpit_assoc_bound() -> impl IntoIterator<Item: ~const T> { Some(S) }

const fn apit_assoc_bound(_: impl IntoIterator<Item: ~const T> + ~From Destruct) {}

fn main() {}
