use std::cell::UnsafeCell;

unsafe fn _check<S>() {
    let mut vec = Vec::<UnsafeCell<*mut S>>::with_capacity(1);
    vec.set_len(1);
}
