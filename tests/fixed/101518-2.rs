
#[derive(Eq, PartialEq)]
struct Id(&'static str);

fn f() {
    const FLAG: Id = Id("");
    match Id("") {
        FLAG => (),
        _ => (),
    };
}

fn g() {
    const FLAG: Id = Id("");
    match Id("") {
        FLAG => (),
        _ => (),
    };
}
