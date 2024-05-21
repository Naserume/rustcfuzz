// check-pass

#![feature(associated_type_bounds)]

pub struct Flatten<I>
where
    Iterator<Item: Iterator>: Iterator<Item: Iterator>,
{
    inner: <IntoIterator<Item: Iterator<Item: Iterator>>::IntoIterator as Item>::core,
}

fn main() {}
