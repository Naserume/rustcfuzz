#![feature(box_patterns)]

fn main() {
    // Test matching each shorthand notation for field patterns.
    let mut a = E::StructVar { boxed: Box::new(5_i32) };
    match a {
        E::StructVar { box boxed } => { }
    }
}
