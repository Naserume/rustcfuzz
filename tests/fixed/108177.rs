#![feature(non_lifetime_binders)]
#![allow(incomplete_features)]
#![crate_type = "lib"]

pub fn f<T>() where for<U> (T, U): Copy {}
