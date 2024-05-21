fn test(s: &Self::Id) {
    match &s[0..3] {
        b"aba" => {}
        _ => {}
    }
}

fn main() {}
