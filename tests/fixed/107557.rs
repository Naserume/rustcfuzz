use std::future;

fn func<T>() -> impl Sized {}

trait Trait<'a> {
    type Assoc;

    fn call() {
        let _ = async {
            let _value = func::<Self::Assoc>();
            future::ready(()).await
        };
    }
}

impl Trait<'static> for () {
    type Assoc = ();
}

fn main() {
    <()>::call();
}
