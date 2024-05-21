#![feature(async_fn_traits)]

use std::future::Future;
use std::ops::AsyncFnMut;

struct Processor;

async fn foo() {
    let value = String::from("hello ice!"); // our owned non-`Copy` value
    
    // let value = &value; // shadow the value with a `Copy` reference to it
    
    // ICE without `value` shadow, compiles with shadow
    Processor.process_asyncfn(|| async move {
        println!("{:?}", value);
        ()
    });
    
    // compile error without shadow, compiles with shadow
    // Processor.process_fn_fut(|| async move {
    //     println!("{}", value);
    //     ()
    // });
}

impl Processor {
    async fn process_asyncfn<'f, F>(&'f mut self, mut f: F) -> ()
    where
        F: AsyncFnMut() -> (),
    { return f().await }
    
    async fn process_fn_fut<'f, F, Fut>(&'f mut self, mut f: F) -> ()
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = ()>,
    { return f().await }
}
