#![feature(non_lifetime_binders)]
#![crate_type = "lib"]
fn a() where for<const C: usize> [(); C]: Copy {}
