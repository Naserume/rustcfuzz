macro_rules! foo {
    ($ty:ty) => {
        fn foo(_: $ty, _: $ty) {}
    }
}

foo!(_);

fn main() {}

