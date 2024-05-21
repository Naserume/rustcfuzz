trait Foo {
    type Context<'register_something> where Self: 'c;
    fn foo(self) -> Self::Bar;
}

impl Foo for Box<Foo> {
    type Bar = <Self as Foo>::Bar;
    fn foo(self) -> <Self as Foo>::Bar {
        (!0).foo()
    }
}
