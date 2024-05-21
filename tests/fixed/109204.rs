trait Fun {
    type Output;
    fn call(&self) -> Self::Output;
}

impl<'a> Fun for () {
    type Output = &'a str;
    fn call(&self) -> &str {}
}
