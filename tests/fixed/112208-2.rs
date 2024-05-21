pub trait TestTrait<T> {}

impl<T> dyn TestTrait<T>
where
    Self: Sized,
{
    fn other_func() -> dyn TestTrait<T> {
        None::<()>
    }
}

fn main() {}
