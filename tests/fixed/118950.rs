trait ToUnit<'a> {
    type Unit;
}

trait Overlap<T> {}

type Assoc<'a, T> = <*const T as ToUnit<'a>>::Unit;

impl<T> Overlap<T> for T {}

impl<T> Overlap<for<'a> fn(&'a (), Assoc<'a, T>)> for T where
    for<'subset> Fn(Subset<'subset>): Overlap<T>
{
}

#![feature(lazy_type_alias)]
