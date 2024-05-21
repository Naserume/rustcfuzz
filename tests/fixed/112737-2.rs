#![feature(ptr_metadata)]
use core::ptr::Pointee;

fn foo(_: &(impl Pointee<Metadata = ()> + ?Sized)) {}

fn bar(x: &dyn Pointee<Metadata = ()>) {
    foo(x)
}
