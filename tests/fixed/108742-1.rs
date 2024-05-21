#![feature(non_lifetime_binders)]
trait X<'a, T>
where
    for<T> (T,): X<'b, str>,
    for<T> T: Clone,
{
    type U: ?Sized;
     final 
}

pub fn main() {}
