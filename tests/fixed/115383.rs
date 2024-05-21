#![allow(internal_features)]
#![feature(rustc_attrs)]

#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0b01)]
pub struct NonZeroF64(f64);

impl NonZeroF64 {
    pub fn new(_: f64) -> Option<Self> {
        None
    }
}
