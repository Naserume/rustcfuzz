#![feature(closure_lifetime_binder)]
#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

fn T() {
    for<T> || -> () {};
}
