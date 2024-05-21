use std::option::IntoIter;

fn main() {
    const C: fn(IntoIter<i32>) -> IntoIter<i32> = <IntoIter<i32> as IntoIterator>::into_iter;
    C(Some(5).into_iter()); // ICE here
}
