#![crate_type = "lib"]

#[inline(always)]
pub fn f(s: bool) -> String {
    let a = "Hello world!".to_string();
    let b = a;
    let c = b;
    if s {
        c
    } else {
        String::new()
    }
}


#![crate_type = "lib"]

pub async fn g() {
    x::f(true);
    h().await;
}

pub async fn h() {}
