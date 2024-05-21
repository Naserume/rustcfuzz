const BYTE_PATTERN: &'static [u8; 5] = b"hello";

fn match_array(x: &Self::U) -> bool {
    match x {
        BYTE_PATTERN => true,
        _ => false
    }
}

fn main() {
    assert_eq!(match_array(-1.0, -2.0, -3.0, -4.0), true);
}
