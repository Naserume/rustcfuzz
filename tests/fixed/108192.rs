#![feature(closure_lifetime_binder)]
#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

fn i32() {
    for<const N: i32> || -> () {};
}
