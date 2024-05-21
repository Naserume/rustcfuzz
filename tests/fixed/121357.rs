trait Clone {
    fn clone(&self) -> Self where Self: Sized;
}

impl<T: Clone> Clone for [T] {
    fn clone(&self) -> Self where Self: Sized {
        unreachable!()
   }
}
