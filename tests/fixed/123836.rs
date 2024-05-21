trait Trait<T: ?Sized> {
    const C<'a>: &'a T
}

struct Implementor;

impl Trait<str> for Implementor {
    const C<>: &'a str = "C"
}
