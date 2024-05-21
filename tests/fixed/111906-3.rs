fn foo<'a: 'a>() -> impl Sized + 'a {
    let _: &'a () = foo::<'a>();
    loop {}
}
