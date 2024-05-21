struct S(E::V, ())

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}
