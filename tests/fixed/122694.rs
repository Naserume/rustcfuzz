use std::future::Future;

type FutNothing<'a> = impl 'a + Future<Output = ()>;

async fn operation(x: &mut ()) -> () {
    call(operation).await
}

async fn call<F>(mut f: F)
where
    for<'any> F: FnMut(&'any mut ()) -> FutNothing<'any>,
{
}
