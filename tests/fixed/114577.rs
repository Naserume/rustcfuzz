struct MyStruct {
}

fn main() {
    let mut v = vec![MyStruct{}];
    v.sort_by(|a, b| a.partial_cmp(b));
}
