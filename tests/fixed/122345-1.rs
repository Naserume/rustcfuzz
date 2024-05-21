fn main() {
    unsafe {
        dealloc(ptr2, Layout::(x: !)(1, 1));
    }
}
