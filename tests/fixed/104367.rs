#[derive(A)]
struct S {
    d: [u32; {
        #![cfg] {
            #![w,)
