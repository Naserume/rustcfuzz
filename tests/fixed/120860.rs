#![feature(non_lifetime_binders)]

fn walk2<'a, T: 'a>(_: T)
where
    for<const C: usize> [(); C]: 'a, // replacing C by 1 fixes ice
{}

fn main() {}
