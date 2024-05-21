#![feature(trait_alias)]
use core::ops::Add;

pub trait DoSome<T> {}

// Option 1 - Compiler panic
pub trait Cell<T: Add<T, Output=T>> = DoSome<T>;

// Option 2 - Compiles
// pub trait Cell<T: Add<T, Output=T>>: DoSome<T> {}
// impl<T,A> Cell<T> for A
// where
//     T: Add<T,Output=T>,
//     A: DoSome<T>,
// {}

struct _Container<T> {
    pub cells: dyn Cell<T>,
}


fn main() {}
