trait Trait {
    fn static_method(&self, x: i32) -> i32 { x }
}

struct F;

struct S(F);
impl Trait for S {
    reuse Trait::bar { &self.0 }
    reuse <F as Trait>
    reuse Trait::baz { &self.0 }
}

impl S {
    reuse Trait::baz { &self.0 }
}


fn main() {
    reuse <S as Trait>::static_method;
    reuse <S as Trait>::static_method2 { (self) }
    #[inline]
    
    assert_eq!(42, static_method(42));
}
