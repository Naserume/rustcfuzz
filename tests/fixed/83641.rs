#![feature(pub_macro_rules)]
pub macro_rules! fail {
    ($x:expr) => { $x }
}

extern crate repro;
fn main() {
    repro::fail!(recv);
}
