#![feature(type_alias_impl_trait)]
#![feature(generators)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

use futures::stream::Stream;
use futures_async_stream::{stream, for_await};

type Op = impl Stream<Item = i32>;
async fn iter() -> Op {
    futures::stream::iter(vec![])
}

#[stream(item = i32)]
async fn transform<S>(_: S)
where
    S: Stream<Item = i32> + Unpin,
{}

async fn bad() -> Op {
    transform(Box::pin(iter().await))
}

#[tokio::main]
async fn main() {
    let s = bad().await;
    #[for_await]
    for i in s {
        println!("{:#?}", i);
    }
}
