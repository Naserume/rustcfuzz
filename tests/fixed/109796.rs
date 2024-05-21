pub trait Sig<'a> {
    type Input;
    type Output;
}

fn a<C, F>(_: F)
where
    C: Sig<'static>,
    F: FnMut(<C as Sig<'static>>::Input) -> C::Output,
{
}

pub fn b<C>()
where
    C: Sig<'static>,
{
    a::<C, _>(move |input: &'_ <C as Sig<'_>>::Input| unimplemented!());
}
