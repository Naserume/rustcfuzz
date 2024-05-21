struct Foo(pub str);

impl Foo {
    fn print(&self) {
        match self {
            &Foo(ref s)  => println!("f\"{}\"", s),
        }
    }
}

fn main() {}
