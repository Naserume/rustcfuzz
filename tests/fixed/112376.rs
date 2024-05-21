use std::borrow::{Cow, ToOwned};
use std::default::Default;

pub struct XorShiftRng;
impl Rng for XorShiftRng {}
pub trait Rng {}
pub trait Rand: Default + Sized {
    fn rand<R: Rng>(_rng: &mut R) -> Self { Default::default() }
}
impl Rand for i32 { }

pub trait IntoCow<'a, B: ?Sized> where B: ToOwned {
    fn into_cow(self) -> Cow<'a, B>;
}

impl<'a> IntoCow<'a, str> for String {
    fn into_cow(self) -> Cow<'a, str> {
        Cow::Owned(self)
    }
}

#[derive(PartialEq, Eq)]
struct Newt<T>(T);

fn eq<T: Eq>(a: T, b: T) -> bool { a == b }


trait Size: Sized {
    fn size() -> usize { std::mem::size_of::<Self>() }
}
impl<T> Size for T {}



#[derive(PartialEq, Eq)]
struct Foo<T>(T);


macro_rules! tests {
    ($($expr:expr, $ty:ty, ($($test:expr),*);)+) => (pub fn main() {$({
        static S: $ty = $expr;
        assert!(eq(S($($test),*), $expr($($test),*)));
    })+})
}

tests! {
    <Vec<()>>::with_capacity, fn(usize) -> Vec<()>, (5);  
}
