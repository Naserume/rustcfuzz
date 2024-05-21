#![feature(negative_bounds, negative_impls)]
#![allow(incomplete_features)]

fn main() {
    make().consume();
}

fn make() -> impl !Trait {}

trait Trait {
    fn consume(self);
}

impl !Trait for () {}
