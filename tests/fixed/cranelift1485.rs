#![feature(thin_box)]

use std::boxed::ThinBox;
use std::error::Error;

use std::fmt;

fn main() {
    let expected = "Foo error!";
    let a: ThinBox<dyn Error> = ThinBox::new_unsize(Foo(expected));
}

#[derive(Debug)]

struct Foo(&'static str);

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for Foo {}
