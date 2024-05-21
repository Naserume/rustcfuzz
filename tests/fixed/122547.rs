#[repr(align(4))]
enum AlsoAlign16 {
    Foo { limb_with_noniche16: NoNiche16 },
    Bar,
}

struct NoNiche16(u64, u64);

fn main() {}
