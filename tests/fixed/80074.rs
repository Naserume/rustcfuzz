macro_rules! foo_ { () => {}; }
use foo_ as foo;

#[macro_use]
extern crate test_project;

fn main() {
    foo!();
}
