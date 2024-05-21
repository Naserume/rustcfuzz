#![allow(long_running_const_eval)]
const SIZE: usize = 128;
pub const CONST_ARRAY: [u64; SIZE] = expensive_calculation();

const fn expensive_calculation() -> [u64; SIZE] {
    let mut i = 0;
    while i < 10_000_000 {
        i += 1;
    }

    [0; SIZE]
}

pub fn dbg_0<const FOO: bool>() {
    dbg!(CONST_ARRAY[0]);
}
