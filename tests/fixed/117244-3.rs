#![feature(effects)]
#![feature(const_trait_impl)]

trait Foo {}

const fn check<T: ~const Foo>() {}

fn main() {}
