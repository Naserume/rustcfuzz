[package]
name = "abc"
version = "0.0.1"
edition = "2018"

mkdir -p src

fn main() {
    println!("Hello, world!");
}

cargo test

/// Examples:
/// ```
/// assert_eq!(fun(), 5);
/// ```
fn fun() -> u8 {
    5
}

cargo test


[[bin]]
name = "icebin"
path = "src/bin.rs"

[lib]
name = "icelib"
path = "src/lib.rs"

fn main() {
    println!("Hello, world!");
}

/// Examples:
/// ```
/// assert_eq!(icelib::fun(), 5);
/// ```
pub fn fun() -> u8 {
    5
}
