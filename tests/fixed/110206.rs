#![crate_type = "lib"]
#![feature(transmutability)]


mod assert {
    use std::mem::BikeshedIntrinsicFrom;

    pub fn is_transmutable<
        Src,
        Dst,
        Context,
        const ASSUME_ALIGNMENT: bool,
        const ASSUME_LIFETIMES: bool,
    >()
    where
        Dst: BikeshedIntrinsicFrom< 
            Src,
            Context,
            (),
            (),
        >,
    {}
}
