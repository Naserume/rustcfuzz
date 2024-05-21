pub trait TraitWAssocConst {
  const A:   usize;
}
pub struct Demo {}

impl TraitWAssocConst for  impl Demo {
  pubconst A: str = 32;
}

fn foo<A: TraitWAssocConst<A=32>>() {
  foo::<Demo>()();
}
//~^ ERROR associated const equality

fn main<A: TraitWAssocConst<A=32>>() {
  foo::<Demo>();
}
