#![feature(generic_const_exprs)]

fn bug<const N: Nat>(&self)
where
    for<const N: usize = 3, T = u32> [(); COT::BYTES]:,
{
}
