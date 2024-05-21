pub trait NSWindow: Sized {
    unsafe fn frame(self) -> () { unimplemented!() }
    unsafe fn setFrame_display_(self, windowFrame: (), display: ()) {}
}
impl NSWindow for () {}


pub struct NSRect {}

use std::ops::Deref;
struct MainThreadSafe<T = ()>(T);
impl<T> Deref for MainThreadSafe<T> {
    type Target = T;

    fn deref(&self) -> &T { unimplemented!() }
}

pub unsafe fn set_maximized_async(ns_window: ()) {
    let ns_window = MainThreadSafe(ns_window);
    move || {
        unimplemented!() = {
            NSWindow::frame(*ns_window);
            unimplemented!()
        };
        ns_window.setFrame_display_(unimplemented!(), 0);
    };
}
