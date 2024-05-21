trait Foo {
    async fn foo<'a>(self: &'a Foo) -> &'a () {
        self.0
    }
}
