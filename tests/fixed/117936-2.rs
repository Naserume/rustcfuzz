// edition: 2021
// compile-flags: --crate-type=lib

use super::A;

mod b {
    pub trait A {}
    // pub trait B {}
}

/// [`A`]
pub use b::*;

fn main() {}
