/*
 error: internal compiler error: broken MIR in Item(DefId(0:13 ~ tabula[7834]::tr2::{impl#0}::get)) (after phase change to runtime-optimized) at bb0[1]:
                                Alias(Opaque, AliasTy { args: [], def_id: DefId(0:18 ~ tabula[7834]::tr2::X::{opaque#0}) }) does not have fields

*/
// compile-flags: -Z mir-opt-level=3
#![feature(type_alias_impl_trait)]
#![crate_type = "lib"]
pub trait Tr {
    fn get(&self) -> u32;
}

impl Tr for (u32,) {
    #[inline]
    fn get(&self) -> u32 { self.0 }
}

pub fn tr1() -> impl Tr {
    (32,)
}

pub fn tr2() -> impl Tr {
    struct Inner {
        x: X,
    }
    type X = impl Tr;
    impl Tr for Inner {
        fn get(&self) -> u32 {
            self.x.get()
        }
    }

    Inner {
        x: tr1(),
    }
}
