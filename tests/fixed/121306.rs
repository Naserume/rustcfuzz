#![feature(async_closure)]

#[allow(unused_must_use)]
fn foo(f: impl async FnOnce()) {
    f();
}

pub fn main() {
    foo(|| async move {});
}
