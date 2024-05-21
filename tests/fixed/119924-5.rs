#![feature(const_trait_impl, effects)]

pub struct S;
struct Expr<const N: u32>;

#[const_trait]
trait Trait {
    fn required(_: Expr<{
        impl S {
            fn perform<T: ~const Trait>() {} // should've gotten rejected during AST validation
            //~^ ICE no host param id for call in const yet no errors reported
        }
	0
    }>);
}
