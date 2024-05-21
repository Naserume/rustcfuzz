#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]

type Fut<'a> = impl Iterator<Item = Result<Signature, ()>>;

trait Trait<'x> {
    type Thing;
}

impl Foo<'_> {
    fn make_fut(&self) -> Box<dyn for<'a> Trait<'a, Thing = Fut<'a>>> {
        todo!();
    }
}

fn main() {}
