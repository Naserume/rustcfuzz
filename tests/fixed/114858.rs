#![feature(impl_trait_in_assoc_type)]

use std::{future::Future, convert::Infallible};

trait AsyncFn<Arg> {
    type Output;
    type Future<'f>: Future<Output = Self::Output>
    where
        Arg: 'f,
        Self: 'f;

    fn call<'s>(&'s self, arg: Arg) -> Self::Future<'s>
    where
        Arg: 's;
}

trait AsyncFn2<Arg> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, arg: Arg) -> Self::Future;
}

impl<F, A, B, Fut> AsyncFn2<(A, B)> for F
where
    F: Fn(A, B) -> Fut,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future = Fut;

    fn call(&self, (a, b): (A, B)) -> Self::Future {
        self(a, b)
    }
}

struct Func<F>(F);

impl<Arg, F, Fut> AsyncFn<Arg> for Func<F>
where
    F: Fn(Arg) -> Fut,
    Fut: Future,
{
    type Output = Fut::Output;
    type Future<'f> = impl Future<Output = Self::Output> + 'f where Arg: 'f, Self: 'f;

    fn call<'s>(&'s self, arg: Arg) -> Self::Future<'s>
    where
        Arg: 's,
    {
        async move { (self.0)(arg).await }
    }
}

struct EnclosedFn<T1, T2> {
    this: T1,
    other: T2,
}

impl<Arg, T1, T2, O> AsyncFn<Arg> for EnclosedFn<T1, T2>
where
    T1: AsyncFn<Arg>,
    T2: for<'s> AsyncFn2<(&'s T1, Arg), Output = O>,
{
    type Output = O;
    type Future<'f> = impl Future<Output = Self::Output> + 'f where Arg: 'f, Self: 'f;

    fn call<'s>(&'s self, arg: Arg) -> Self::Future<'s>
    where
        Arg: 's,
    {
        self.other.call((&self.this, arg))
    }
}

trait Enclosed<Arg>: AsyncFn<Arg> {
    fn enclosed<T>(self, other: T) -> EnclosedFn<Self, T>
    where
        T: for<'s> AsyncFn2<(&'s Self, Arg)>,
        Self: Sized,
    {
        EnclosedFn { this: self, other }
    }
}

impl<Arg, F> Enclosed<Arg> for F where F: AsyncFn<Arg> {}

fn main() {
    async fn middelware<S, Arg>(s: &S, arg: Arg) -> S::Output
    where
        S: AsyncFn<Arg>,
    {
        s.call(arg).await
    }

    let f = Func(|arg: String| async move { Ok::<_, Infallible>(arg) })
        .enclosed(middelware)
        .enclosed(middelware);

    futures::executor::block_on(f.call(String::new())).unwrap();
}
