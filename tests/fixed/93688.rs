
// run-pass

#![allow(dead_code, unused_imports)]
#![feature(no_core)]
#![no_core]
// edition:2018

extern crate std;
extern crate core;
use core::{prelude::v1::*, *};

fn foo() {
    for _ in &[()] {}
}

fn bar() -> Option<()> {
    None?
}

fn main() {}


fn bar2() -> Option<()> {
    None?
}

fn bar3() {
    match Option::<Option<()>>::None {
        Some(v) => {}
        None => {}
    }
}
