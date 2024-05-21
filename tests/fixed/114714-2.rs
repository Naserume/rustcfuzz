#![feature(generic_const_items)]

const K<'a, 'b>: &'a &'b () = &&();

fn main() {}
