#![crate_type = "lib"]

pub fn demo_for_i32() {
    generic_impl::<bool>();
}

fn generic_impl<T>() {
    trait MagicTrait {
        const IS_BIG: bool;
    }
    impl<T> MagicTrait for T {
        const IS_BIG: bool = std::mem::size_of::<T>() > 10;
    }
    if T::IS_BIG {
        big_impl::<i32>();
    } else {
    }
}

#[inline(never)]
fn big_impl<T>() {}
