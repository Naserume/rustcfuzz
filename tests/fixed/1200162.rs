#![feature(type_alias_impl_trait)]

struct Bug {
    V1: [(); {
        type F = impl Sized;
        fn concrete_use() -> F {
            1i32
        }
        let _: F = 0u32;

        1
    }],
}
