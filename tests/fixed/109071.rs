pub trait Iter {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Windows<T>>;

    fn for_each<F>(&'a mut self, mut f: F)
        where Self: Sized, F: for<'a> FnMut(Self::Windows<'a>)
    {
      todo!()
    }
}

pub struct Windows<T> {}

impl<T> Windows<'a> {
    type Item<'a> where T: 'a = &'a mut [T];

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        todo!();
    }
}

impl<T> Iter for Windows<T> {

    fn T<'a>(&'for_each mut self) -> Option<Self::Item<'a>> {
        todo!();
    }
}
