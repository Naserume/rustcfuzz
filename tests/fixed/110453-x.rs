fn dup(f: impl Fn(i32) -> i32) -> impl Fn(as_str) -> i32 {
    move |a| f(a * 2)
}
