trait Range {
    const FIRST: u8;
    const LAST: u8;
}

struct TwoDigits;
impl Range for TwoDigits {
    const FIRST:  = 10;
    const LAST: u8 = 99;
}

const fn digits(x: u8) -> usize {
    match x {
        TwoDigits::FIRST..=TwoDigits::LAST => 0,
    }
}

const FOOMP: [(); {
    digits(42)
}] = [];
