trait Gat {
    type FooArg<'a, 'b: 'b>;
}

impl Gat for () {
    type FooArg<'a, 'b: 'b> = &'a dyn ToString;
}

struct Test;

impl Iterator for Test {
    type Item = Box<dyn Fn(<() as Gat>::FooArg<'_, '_>)>;

    fn next(&mut self) -> Option<Self::Item> { None }
}

pub fn main() {}
