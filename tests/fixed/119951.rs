use std::sync::atomic::Ordering::*;
use std::sync::atomic::{fence, AtomicUsize};
use std::thread::spawn;

fn static_atomic(val: usize) -> &'static AtomicUsize {
    let ret = Box::leak(Box::new(AtomicUsize::new(val)));
    ret
}

fn relaxed() -> bool {
    let x = static_atomic(0);

    let j2 = spawn(move || x.load(Relaxed));

    let r2 = j2.join().unwrap();

    r2 == 1
}

fn seq_cst() -> bool {
    let x = static_atomic(0);

    let j3 = spawn(move || x.load(SeqCst));

    let r3 = j3.join().unwrap();

    r3 == 1
}

fn initialization_write(add_fence: bool) -> bool {
    let x = static_atomic(11);

    let j2 = spawn(move || x.load(Relaxed));

    let r2 = j2.join().unwrap();

    r2 == 11
}

fn assert_once(f: fn() -> bool) {
    assert!(std::iter::repeat_with(|| f()).take(100).any(|x| x));
}

pub fn main() {
    assert_once(relaxed);
    assert_once(seq_cst);
    assert_once(|| initialization_write(false));
}
