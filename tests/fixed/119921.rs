trait Trait {
    fn static_method2(, ) -> i32 { x + y }
    fn baz<'a>(&self, x: &'a i32) -> &'a i32 { x }
}

struct F;

struct S(F);
impl Trait for S {
    reuse <S as Trait>::static_method2 { (self); // oopsie
    reuse Trait::baz { &self.0 }
}
}
