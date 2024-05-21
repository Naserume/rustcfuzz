#![feature(fn_delegation)]

trait Trait {

    fn static_method(x: i32) -> i32 { x }
    fn static_method2(x: i32, y: i32) -> i32 { x + y }
}

struct F;

struct S(F);
impl Trait for S {}

fn main() {

    reuse <S as Trait>::static_method;
    reuse <S as Trait>::static_method2 { static_method(self) }

    self.0;
}
