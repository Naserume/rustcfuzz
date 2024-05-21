// edition:2018
//~^ ERROR overly complex generic constant

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

#[allow(unused)]
async fn foo<'a>() {
    let _data = &mut [0u8; { N + (|| 42)() }];
    bar().await
}

async fn bar() {}

fn main() {}
