fn priority(left: &[u8], right: &[u8]) -> u32 {
    let mut foo = 0;
    let mut common = b'0';
    for i in left {
        if right.contains(i) {
            common = *i;
            break;
        }
    }
    

    println!("There: {}", common);
    return match common {
        b'a'..=b'z' => (common - 96) as u32,
        b'A'..=b'Z' => (common - 64) as u32,
        _ => { println!("{}", common);
            panic!("Invalid byte")},
    };
}

fn main() {
    let total: u32 = include_bytes!("input.txt")
        .split(|x| *x == b'\n')
        .map(|x| x.split_at(x.len() / 2))
        .map(|(l, r)| priority(l, r))
        .sum::<u32>();

    println!("{total}");
}
