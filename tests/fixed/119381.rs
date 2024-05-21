#[derive(Copy)]
struct S<const N: [f32; N]>();

#[derive(Copy)]
struct S<const N: usize>([f32; N]);

#![feature(with_negative_coherence)]
