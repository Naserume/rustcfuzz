pub fn main() {
    fn baz(&self) {}
    let _ = &baz as &dyn Fn(i32);
}
