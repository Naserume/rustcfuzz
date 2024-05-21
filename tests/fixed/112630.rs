enum Foo {
    Bar(B),
}

trait Bar {}

impl Bar for [u32; Foo::Bar as usize] {}
