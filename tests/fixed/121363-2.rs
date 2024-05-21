#[derive(Debug)]
struct Foo(i32);

#[derive(Debug)]
struct TwoStrs(str, str) where str: Sized;
