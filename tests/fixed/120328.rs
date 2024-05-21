fn main() {
    for item in &[1, 2, 3] {
        let _ = || *item;
    }
}
