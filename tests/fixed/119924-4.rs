#![feature(const_trait_impl, effects)]

pub struct S;
#[const_trait]
trait Trait {
    fn provided() {
        impl S {
            fn perform<T: ~const Trait>() {} // should've gotten reject during AST validation
            //~^ ICE no host param id for call in const yet no errors reported
        }
    }
}
