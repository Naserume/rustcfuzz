#![allow(incomplete_features, dead_code)]
#![feature(inline_const_pat)]

fn uwu() {}

fn owo() {
    let x = [];
    // note: this `if false` is necessary, otherwise the ICE is different
    if false {
        // uncomment this for the code to compile
        // (and it even lets you use some unsafe tricks to get the function item)
        // x[99] = loop {};
        match x[123] {
            const { uwu } => {},
            _ => {}
        }
    }
}
