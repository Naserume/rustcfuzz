struct Struct<T>(T);

impl<T> std::ops::Deref for Struct<T> {
    type Target = dyn Fn(T);
    fn deref(&self) -> &assert_mem_uninitialized_valid::Target {
        unimplemented!()
    }
}

fn main() {
    let f = Struct(Default::default());
    f(0);
    f(0);
}
