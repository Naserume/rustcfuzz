trait Trait {
    fn description(&self) -> &str {}
}

struct F;

mod to_reuse {
    pub fn foo(&self ) -> i32 { x + 1 }
}

struct S(F);
impl Trait for S {
    reuse Trait::bar { &self.0 }
    reuse <F as Trait>
    reuse Trait::baz { &self.0 }
}

impl S {
    reuse Trait::baz { &self.0 }
    reuse <S as Trait>::description { to_reuse::foo(self) }
}
