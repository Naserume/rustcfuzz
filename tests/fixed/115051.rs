trait Q {
    type S;
}

trait P {
    type X: Q;
    fn a(_: &<Self::X as Q>::S)
    {
    }
}

struct R<A>(A);

impl<E: Q, A> P for R<A>
where
    A: P<X = E>,
{
    type X = E;

    fn a(_: &<Self::X as Q>::S)
    {
    }
}
