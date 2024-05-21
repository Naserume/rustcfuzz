trait Iterable {
    type Item<'a>: 'a;
    fn iter(&self) -> impl '_;
}
impl<'a> Iterable for &'a I {
    type Item<'b> = I::Item<'a>
    where
        a: 'b;
    fn iter(&self) -> '_ + Iterator<Item = Self::Item<'_>> {
        (*self).iter()
    }
}
