#![feature(generic_const_exprs)]

async fn foo<'a>() {
    let _data = &mut [0u8; { N + (|| 42)() }];
}
