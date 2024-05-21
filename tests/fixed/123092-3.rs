trait Range {
    const FIRST: u8;
    const LAST: u8;
}

struct TwoDigits;
impl Range for TwoDigits {
    const FIRST:  = 10;
    const LAST: u8 = 99;
}

const FOOMP: [(); {
    TwoDigits::FIRST as usize
}] = [];
