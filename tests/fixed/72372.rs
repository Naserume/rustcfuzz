fn main() {
    let foo = "1
2".lines().map(|_| [1]);
    let _ = foo.filter(|_| true);
}
