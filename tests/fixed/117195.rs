#![feature(unboxed_closures)]
use std::future::Future;

async fn wrapper<F>(f: F)
where
    for<F> F:,
    for<'a> <i32 as FnOnce<(&'a mut i32,)>>::Output: Future<Output = ()> + 'a,
{
    let mut i = 41;
    &mut i;
}
