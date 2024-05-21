struct Checked<const F: fn(usize) -> bool>;
fn not_one(val: usize) -> bool { val != 1 }
fn not_two(val: usize) -> bool { val != 2 }
fn main() {
    let _ = Checked::<{const _: Checked<not_one> = Checked::<not_one>;
    let _: Checked<not_one> = Checked::<not_two>;
    let _: Checked<not_one> = Checked}>;
    
}
