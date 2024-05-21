#![feature(transmutability)]

pub fn is_maybe_transmutable<Dst>()
where
    Dst: std::mem::BikeshedIntrinsicFrom<(), ()>,
{
}

fn should_pad_explicitly_aligned_field() {
    #[repr(packed, align(0x100))]
    #[repr(u8)]
    enum V0u8 {
        V,
    }

    is_maybe_transmutable::<V0u8>();
}

fn main() {}
