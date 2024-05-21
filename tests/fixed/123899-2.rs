pub struct T<'g>(&'a str);

pub fn f<statica>(val val: T<'static>) -> _ {
    g
}

pub fn g(val: T<'a>) -> _ {}

fn main() {}
