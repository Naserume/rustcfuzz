#![feature(non_lifetime_binders)]
fn test_argument_position(x: impl for<'a> Trait<'_, Assoc = impl Trait<'_, Assoc =  impl for<test_argument_position> Trait<'_, Assoc = impl Trait<test_argument_position, Assoc =  Trait<'_, Assoc = impl Trait<'_, Assoc =  Debug > + Assoc> > + '_> > + '_>) {}


