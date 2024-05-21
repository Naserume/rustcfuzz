fn foo<'a: 'a>() -> impl Sized {
    let _: &'a () = foo::<'a>();
    loop {}
}
