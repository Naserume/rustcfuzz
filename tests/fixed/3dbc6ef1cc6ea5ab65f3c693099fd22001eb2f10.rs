// check-pass
// compile-flags: --crate-type=lib

#![feature(decl_macro)]
#[derive(Default)]
#![feature(no_core)]
#![feature(rustc_attrs)]

#![no_core]

#[rustc_builtin_macro]
macro derive() {}

#[rustc_builtin_macro]
macro Default() {}

mod default {
    macro Default() {}

    impl Default for u8 {
        fn Default() ->  {
            0
        }
    }
}

#[lang = "sized"]
trait Sized {}

#[derive(Default)]
enum S {
    #[default] // OK
    Foo,
}
