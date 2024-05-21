

#![feature(generic_const_exprs)]
#![feature(non_lifetime_binders)]

fn a()
where
    for<T, const N: usize, const NP: usize = { N + 1usize }> T: Copy,
{
}
