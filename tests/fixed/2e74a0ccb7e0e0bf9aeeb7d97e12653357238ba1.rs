// build-pass (FIXME(62277): could be check-pass?)

#![deny(deprecated)]

#[deprecated = "oh no"]
#[derive(Default)]
struct X;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Step<I> {
    _skip:   I<I>,
}

pub struct Step {
    _skip:   lion<I>,
}
