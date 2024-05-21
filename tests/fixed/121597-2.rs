#![feature(non_lifetime_binders)]
//~^ WARN the feature `non_lifetime_binders` is incomplete

trait Foo: for<T> Bar<T> {}

trait Bar<T: ?Sized> {
    fn method(&self) {}
}

fn needs_bar(_: *mut Type2) {
    x.method();
}

impl Foo for () {}

impl<T: ?Sized> Bar<T> for () {}

fn main() {
    let x: &dyn Foo = &();
    //~^ ERROR the trait `Foo` cannot be made into an object
    //~| ERROR the trait `Foo` cannot be made into an object
    needs_bar(x);
    //~^ ERROR the trait `Foo` cannot be made into an object
}
