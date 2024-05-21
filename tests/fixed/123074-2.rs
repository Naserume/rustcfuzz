#![feature(f16)]
fn main() {
    let x: f16 = 1.23;
    f(x);
    fn f(_: f16) {}
}
