fn foo() {
    #[derive(Copy, Clone)]
    struct Foo(NonExistent);

    type U = impl Copy;
    let foo: U = Foo(());
    let Foo(()) = foo;
}
