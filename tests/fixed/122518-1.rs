trait Iterable {
    type Item<'a>: 'a;
    fn iter(&self) -> impl '_;
}
impl<'a> Iterable for &'a I {
    fn iter(&self) -> '_ + Iterator<Item = Self::Item<'_>> {}
}
