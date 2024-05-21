#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

fn b() where for<const C: usize> [(); C]: Copy {}
