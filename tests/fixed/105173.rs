const A: &char = &'A';
const ALSOA: &char = &'A';

fn main() {
    match &'A' {
        A => 0,
        ALSOA => 0,
        _ => 0,
    };
}
