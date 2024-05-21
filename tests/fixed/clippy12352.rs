//@ run-pass
#![allow(unused_variables)]
enum Foo {
    Bar = (|_: T::Item| { }, 42).1,
}

fn main() {
    assert_eq!(Foo::Bar as usize, 42);
}
