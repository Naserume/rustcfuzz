use std::mem::{self};

struct Wrap<T> {
    wrapped: T,
}

fn generic<T: 'static>() {
    unsafe {
        let _val: Wrap<&'static T> = mem::uninitialized();
    }
}

fn main() {}
