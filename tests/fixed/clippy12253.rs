const C: [u32; 5] = [0; 5];

#[allow(unconditional_panic)]
fn test() -> u32 {
    C[0xfffffe7fffffffffffffffffffffffff]
}

fn main() {
    test();
}
