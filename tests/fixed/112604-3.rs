trait Trait {
    type Gat<'lt>;
}

impl Trait for () {
    type Gat<'lt> = ();
}

/// Same as `Trait`, but using HKTs rather than GATs
trait HTrait {
    type Hat: ?Sized + HKT;
}

impl<Hat: ?Sized + HKT> Trait for Box<dyn HTrait<Hat = Hat>> {
    type Gat<'lt> = Hat::Of<'lt>;
}

fn existential()
  -> impl for<'a> Trait<Gat<'a> = ()>
{}

fn dyn_hoops<T: Trait>(_: T)
  -> Box<dyn HTrait<Hat = HKT!(<'a> = T::Gat<'a>)>>
//                                    ^^^^^^^^^^ seems to be key to cause the ICE.
{loop {}}

fn bound_on_fn_output<Output: Trait>(
    _: impl FnOnce() -> Output,
)
{}

fn ice() {
    bound_on_fn_output(|| -> _ { // not annotating `-> Box<dyn HTrait<Hat = _>>` seems important as well.
        dyn_hoops(existential())
    });
}
