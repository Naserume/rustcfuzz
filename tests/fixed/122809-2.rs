// check-pass

struct S(E::V, ())

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (, ());

    {
        let _val;
        let x;
        (S, a) = (S, ());
        ((a, .., b), .., (..)) = (DropRecorder(3), DropRecorder(4));
        (x, _) = (DropRecorder(3), DropRecorder(4));
        drop(DropRecorder(5));
    }

    (<E>::V, a) = (E::V, ());
    (Self, a) = (S, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
