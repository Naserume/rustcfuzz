// check-pass

#![feature(generic_const_items, associated_const_equality)]
#![allow(incomplete_features, dead_code)]

trait Owner {
    const K<const N: u16>: u32;
}

impl Owner for () {
    const K<const N: u32>: u32 = N + 1;
}

fn take1(_: impl Owner<K<99> = 100>) {}

fn main() {
    take1(());
}
