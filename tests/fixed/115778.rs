use std::future::Future;

async fn bop() {
    fold(run(), |mut foo| async move {
        &mut foo.bar;
    })
}

fn fold<Fut, F, U>(_: Foo<U>, f: F)
where
    F: FnMut(Foo<U>) -> Fut,
{
    loop {}
}

struct Foo<F> {
    bar: Vec<F>,
}

fn run() -> Foo<impl Future<Output = ()>> {
    loop {}
}
