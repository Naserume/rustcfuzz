#![feature(generic_const_exprs)]

trait If<const COND: bool> {}
impl If<true> for () {}

trait IsZero<const N: u8> {}

impl<const N: usize> IsZero<N> for () where (): If<{ N == 0 }> {}

trait Foobar<const N: u8> {}

impl<const N: u8> Foobar<N> for () where (): IsZero<N> {}

impl<const N: u8> Foobar<N> for () {}

pub fn main() {}
