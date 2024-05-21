#![feature(type_alias_impl_trait)]

trait T {
    type Item;
}

type Alias<'a> = impl T;

struct S;
impl<'a> T for &'a S {
    type Item = &'a ();
}

fn filter_positive<'a>() -> Alias<'a> {
    &S
}

fn with_positive(fun: impl Fn(Alias<'_>)) {}

fn main() {
    with_positive(|_| ());
}
