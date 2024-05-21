#![feature(const_trait_impl)]

#[const_trait]
trait Func {
    fn trigger(self) -> usize;
}

struct Cls;

impl const Func for Cls {
    fn trigger(&self, a: usize) -> usize {
        0
    }
}

enum Bug<T = [(); Cls.trigger()]> {
    V(T),
}
