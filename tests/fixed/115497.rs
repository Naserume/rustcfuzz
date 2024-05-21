#![feature(generic_const_exprs)]
#![feature(non_lifetime_binders)]
#![feature(associated_type_bounds)]

trait TraitA<'a> { type AsA; }
trait TraitB<'a, 'b> { type AsB; }
trait TraitC<'a, 'b, 'c> {}
struct X;
struct Y;
struct Z;

fn foo<T>() where for<const N: u8 = { T::<0>::A as u8 + T::<0>::B as u8 }> T: TraitA<'a, AsA: for<'b> TraitB<'a, 'b, AsB: for<'c> TraitC<'a, 'b, 'c>>> {}

fn main() {}
