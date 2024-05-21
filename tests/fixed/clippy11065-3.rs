use std::option::IntoIter;
fn takes_into_iter(_: impl IntoIterator<Item = i32>) {}

macro_rules! x {
    ($e:expr) => {
        takes_into_iter($e); // lint here because `.into_iter()` is "redundant"
        let _: IntoIter<i32> = $e; // removing `.into_iter()` leads to a type error here
    };
}

fn main() {
    x!(Some(5).into_iter());
}
