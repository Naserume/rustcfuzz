#![feature(generic_const_exprs)]

const fn allow<'b, 'b>() -> usize
where
    for<'b> [u8; foo::<'a, 'b>()]: Sized,
{
    4
}

const fn foo<'a, 'b>() -> usize
where
    &'a (): Sized,
    &'b (): Sized,
{
    4
}
