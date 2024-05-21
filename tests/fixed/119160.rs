use std::hint::black_box;

static F: fn() -> bool = f as fn() -> bool;
static G: fn() -> bool = g as fn() -> bool;

pub fn f() -> bool {
    F == G
}

pub fn g() -> bool {
    F != G
}

fn main() {
    assert_ne!(
        black_box(f as fn() -> bool)(),
        black_box(g as fn() -> bool)(),
        "(F == G) != (F != G)",
    );
}
