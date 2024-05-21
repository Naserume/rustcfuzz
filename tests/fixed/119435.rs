use std::ops::Index;
pub struct v4;
pub trait v7: Index<v4> + Index<v4> {}

pub trait v0 {
type v2: v7;
fn v3(&self) -> &Self::v2;
}
pub fn v1(v8: &impl v0) {
let v3 = v8.v3();
1&v3[v4];
}
fn main() {}
