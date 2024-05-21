fn temp() -> (String, i32) {
    (String::from("Hello"), 1)
}

fn main() {
    let f = if true { &temp() } else { &temp() };
}
