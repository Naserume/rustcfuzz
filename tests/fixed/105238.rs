#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

trait Ret {
    type R;
}

struct Cond<const PRED: bool, U, V>(std::marker::PhantomData<U>, std::marker::PhantomData<V>);

impl<U, V> Ret for Cond<true, U, V> {
    type R = U;
}

impl<U, V> Ret for Cond<false, U, V> {
    type R = V;
}

struct RobinHashTable<const MAX_LENGTH: usize>
where
    Cond<{ MAX_LENGTH < 65535 }, u16, u32>: Ret,
{
    _idx: <Cond<{ MAX_LENGTH < 65535 }, u16, u32> as Ret>::R,
}

fn main() {
    use std::mem::size_of;
    println!("{}", size_of::<RobinHashTable<1024>>());
    println!("{}", size_of::<RobinHashTable<65536>>());
}
