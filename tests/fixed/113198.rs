

#![feature(associated_const_equality)]

fn user() -> impl Owner<i32, C = 0> {}

trait Owner<K> { const C: K; }
impl<K: ConstDefault> Owner<K> for () { const C: K = K::DEFAULT; }

trait ConstDefault { const DEFAULT: Self; }
impl ConstDefault for i32 { const DEFAULT: Self = 0; }

fn main() {}
