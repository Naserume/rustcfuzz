#![feature(return_type_notation)]

trait Super1<'a> {
    fn bar<'a: 'a>() -> bool;
}
impl Super1<bar(): Send> for () {}
