#![allow(incomplete_features, dead_code)]
#![feature(inline_const_pat)]

fn uwu() {}

#[allow(unconditional_panic)]
fn owo() {
    let x = [];
    // uncomment this for the code to compile
    // (and it even lets you use some unsafe tricks to get the function item)
    // x[99] = loop {};
    match x[123] {
        const { uwu } => {}
        _ => {}
    }
}
