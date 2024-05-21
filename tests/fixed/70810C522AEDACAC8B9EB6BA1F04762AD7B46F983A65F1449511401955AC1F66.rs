// run-pass
#![allow(dead_code)]
struct S<T> {
    x: T
}

impl<T> ::std::ops::Drop for S<T> {
    fn drop(&mut self) {
        println!("bye");
    }
}

const fn tester_fn<T>(f: T) -> T::Output
where
    T: ~const Fn<()> + ~const Destruct,
{
    f()
}
