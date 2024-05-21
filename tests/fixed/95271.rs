
enum Foo {
    Foo(i32),
}

fn bar(foo: Foo) {
    || {
        // `let foo = foo;` makes the ICE disappear
        let Foo::Foo(baz) = foo;
    };
}
pub fn main() {}
