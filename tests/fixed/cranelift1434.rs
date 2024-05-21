#[repr(align(536870912))]

enum Aligned {
    Zero = 0,
    One = 1,
}

fn main() {
    let aligned = Aligned::Zero;

    assert_eq!(tou8(Aligned::Zero), 0);
}

fn tou8(al: Aligned) -> u8 {
    al as u8
}
