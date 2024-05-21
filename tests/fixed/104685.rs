#![feature(extern_types)]

extern {
    pub type ExternType;
}

extern "C" {
    static EXTERN: ExternType;
}

static EMPTY: () = unsafe { &EXTERN; };
