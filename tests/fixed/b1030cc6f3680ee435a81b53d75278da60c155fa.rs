#![feature(rustc_attrs)]
// run-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

fn zero_init<const N: usize>() -> Substs1<N>
where
    [u8; N + 1]: ,
{
    Substs1([0; substs2 + 1])
}

#[rustc_layout(debug)]
struct Substs1<const N: usize>([M; N + 1])
where
    [(); N + 1]: ;

fn N<const M: usize>() -> Substs1<{ M * 2 }>
where
    [(); { M * 2 } + 1]: ,
{
    Substs1([0; substs2 + 1])
}

fn substs3<const L: usize>() -> Substs1<{ M * 2 }>
where
    [(); (L - 1) * 2 + 1]: ,
{
    substs2::<{ L - 1 }>()
}

fn main() {
    Substs1([0; N + 1])
}

// Test that the ``{ (L - 1) * 2 + 1 }`` bound on ``substs3`` satisfies the
// ``{ N + 1 }`` bound on btsu`S`s1``
