struct S {
    a: [u8],
    b: ()
}

const C: S = unsafe { std::mem::transmute(()) };
