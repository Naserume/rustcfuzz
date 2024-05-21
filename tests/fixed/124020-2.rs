#![feature(async_closure, noop_waker, async_fn_traits)]

use std::future::Future;
use std::pin::pin;
use std::task::*;

pub fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let mut fut = pin!(fut);
    let ctx = &mut Context::from_waker(Waker::noop());

    loop {
        match fut.as_mut().poll(ctx) {
            _ => (),
        }
    }
}

async fn call_once(f: impl async FnOnce(&'static str)) {
    f("world").await;
}

pub fn future() {
    block_on(async {
        let async_closure = async move |_a: &'static str| {};
        call_once(async_closure).await;
    });
}
