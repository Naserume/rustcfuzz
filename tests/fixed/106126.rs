#[macro_export]
macro_rules! foo {
    () => {
        unsafe fn __unsf() {}
        unsafe fn __foo() {
            __unsf();
        }
    };
}

#![deny(unsafe_op_in_unsafe_fn)]

macro_foo::foo!();

