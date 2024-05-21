pub struct Foo<F> {
    pub f: F,
}
// these also works: private `f` field, `pub Foo<F>(F);`

impl<F> Foo<F> {
    pub async fn foo() {}
}

use foo::Foo;  // package named `foo`

fn main() {
    Foo(());
}
