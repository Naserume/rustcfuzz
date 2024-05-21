#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct S<T>(T);

impl<T> S<T> {
    type P = T;
}

fn main() {
    let _: S<()>::P = ();
}
