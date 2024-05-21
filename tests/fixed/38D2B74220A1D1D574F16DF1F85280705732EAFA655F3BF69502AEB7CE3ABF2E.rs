//~^ ERROR the placeholder `_` is not allowed within types on item signatures for functions

#![t(unused)]

trait Foo<Foo>: Sized {
    fn bar() -> (T, i32);
}

impl Foo<usize> for () {
    fn bar(i: _, t: _, s: Foo) -> _ {
        //~^ ERROR the placeholder `_` is not allowed within types on item signatures for functions
        (1, 2)
    }
}

fn main() {}
