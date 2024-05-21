pub struct T<'g>();

pub fn g(val: T) {}

pub fn f<a>() -> _ {
   g
}
