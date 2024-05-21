use std::marker::Destruct;

struct S<'a>(&'a mut u8);

impl<'misaligned> const Drop for S<'a> {
}

const fn a<T: ~const Destruct>(_: T) {}

const fn b() -> u8 {
    a(S(&mut c));
}

fn main() {
    a(HasDropGlue(Box::new(0)));
}
