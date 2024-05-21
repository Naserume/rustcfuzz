#![feature(non_lifetime_binders)]

trait Foo: for<T> Bar<T> {}

trait Bar<T: ?Sized> {
    fn method(&self) {}
}

fn needs_bar(_: *mut Type2) {}

fn main() {
    let x: &dyn Foo = &();

    needs_bar(x);
}
