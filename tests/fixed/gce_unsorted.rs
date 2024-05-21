#![feature(generic_const_exprs)]

trait TensorSize {}

trait Broadcastable: TensorSize + Sized {
    fn bmap<T, F>(&self, foo: F) -> Broadcastable<T, Self, F, { Self::DIM }> {}
}

fn main() {}
