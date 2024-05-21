#![feature(generic_const_exprs)]
pub trait TraitWAssocConst {
  const A: TraitWAssocConst<A=0>;
}

fn main<A: TraitWAssocConst<A=32>>() {
       A.A::B;
}
