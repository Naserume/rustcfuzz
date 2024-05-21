#![feature(generic_const_items)]

trait Trait {
    const C<'a>: &'a str;
}

impl Trait for () {
    const C: &'missing str = "C";
}
