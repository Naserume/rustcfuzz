use core::ops::Index;

struct Map<T, F> {
    f: F,
    inner: T,
}

impl<T, F, Idx> Index<Idx> for Map<T, F>
where
    T: Index<Idx>,
    F: Fn(&T, Idx) -> Idx,
{
    type Output = T::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        self.inner.index(index)
    }
}

fn main() {
    Map {
        inner: [0_usize],
        f: |_, i: usize| 1_usize,
    }[0];
}
