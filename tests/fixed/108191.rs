#![feature(non_lifetime_binders)]
#![feature(generic_const_exprs)]
#![crate_type = "lib"]

fn foo<T>(ExactSizeIterator: T) -> usize
where
    for<'a> &'a T:  ,
    for<T> <&'a [<&'a [u32; 1] as IntoIterator>::IntoIter; ExactSizeIterator: T] as IntoIterator>::IntoIter: ExactSizeIterator,
{}
