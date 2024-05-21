trait Trait<T: ?Sized> {}

impl<T: ?Sized> Trait<T> for i32 {}

fn produce() -> impl for<'this, Args> Trait<(), Assoc = impl Trait<T>> {
    16
}
