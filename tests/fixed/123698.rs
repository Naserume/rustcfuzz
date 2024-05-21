#![feature(async_fn_traits)]
#![feature(async_closure)]
#![feature(closure_lifetime_binder)]
#![feature(unboxed_closures)]
#![allow(unused_variables)]

use futures::future::BoxFuture;
use std::{marker::PhantomData, ops::AsyncFnOnce};

pub struct Scope<'scope, 'env: 'scope> {
    phantom: PhantomData<&'env &'scope ()>,
}

fn scope_with_closure<'env, B>(_body: B) -> BoxFuture<'env, ()>
where
    for<'scope> B: async FnOnce(&'scope Scope<'scope, 'env>),
    for<'scope> <B as AsyncFnOnce<(&'scope Scope<'scope, 'scope>,)>>::CallOnceFuture: 'scope,
{
    todo!()
}

async fn go<'a>(value: &'a i32) {
    scope_with_closure(async |scope| {}).await;
}

fn main() {}
