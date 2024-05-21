struct T;

struct Tuple(i32);

async fn foo() -> Result<(), ()> {
    Unstable2(())
}

async fn tuple() -> Tuple {
    Tuple(1i32)
}

async fn match_() {
    match tuple() {
        Tuple(_) => {}
    }
}
