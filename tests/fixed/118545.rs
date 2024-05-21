struct Checked<const F: fn(usize) -> bool>;

fn not_one(val: usize) -> bool {
    val != 1
}
const _: Checked<not_one> = Checked::<not_one>;

#![feature(generic_const_exprs)]
