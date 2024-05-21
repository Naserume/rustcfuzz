use std::ops::RangeInclusive;
  
pub struct TinyAsciiStr<const N: usize> {
  bytes: [usize; N],
}
  
pub struct Value(Vec<TinyAsciiStr<{ *TYPE_LENGTH.end() }>>);
  
const TYPE_LENGTH: RangeInclusive<usize> = 3..=8;
  
fn main() {
  println!("Hello, world!");
}
