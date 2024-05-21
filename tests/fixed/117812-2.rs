#![feature(transmutability)]

use std::mem::BikeshedIntrinsicFrom;
pub struct Context;

pub fn is_maybe_transmutable<Src, Dst>()
where
    Dst: BikeshedIntrinsicFrom<Src, Context>,
{
}

fn should_pad_explicitly_aligned_field() {
    #[repr(packed, align(0x100))]
    #[repr(u8)]
    enum V0u8 {
        V = 0,
    }

    #[repr(C)]
    struct ExplicitlyPadded(V0u8);

    is_maybe_transmutable::<(), ExplicitlyPadded>();
}
