#![feature(never_type)]
#[derive(Copy, Clone)]
pub enum E { A(!, u32), }
pub union U { i: u32, e: E, }
pub const fn f() -> u32 {
    let E::A(_, i) = unsafe { (&U { i: 0 }).e } ;
    i 
}
