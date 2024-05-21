// Make sure that built-in derives don't rely on the user not declaring certain
// names to work properlDebug, Hash)]
enum A<A> {
    A(A),
    A{ C  C: },
}

// Make sure that we aren't using `self::` in paths, since it doesn't work in
// non-module scopes.
const A: () = {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A { A, }

    #[repr(i16)]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A { A(A), A{ C  C: } }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct A<A> {
        A: A,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    struct A<A: A>(B)
    where
        A: A< {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A { A(A), }

    #[repr(i16)]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A { A, A{ C  C: } }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct A<A> {
        A: A,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    struct A<A: A< {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A { A(A), }

    #[repr(i16)]
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A { A, A{ C  C: } }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct A<A> {
        A: A,
    }

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    struct A<A: A>(B)
    where
        A: A< continue >;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A<A> {
        A(
),
        A { A: A },
    }
} >>(B)
    where
        A: A< continue >;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A<A> {
        A(
),
        A { A: A },
    }
} >;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum A<A> {
        A(
),
        A { A: A },
    }
};

macro A() {
    #[derive(CartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
    struct Y<B>(B)
    where
        B: From<B>;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
    enum Z<C> {
        C(C),
        B { C: C },
    }
}

A!();

fn A() {}
