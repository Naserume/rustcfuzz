#![feature(lazy_type_alias)]

impl Trait for Struct {}
trait Trait {
    fn test(&self) {}
}

type Struct = dyn Trait + Send;

fn main() {}
