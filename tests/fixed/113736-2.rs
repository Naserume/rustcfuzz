#![feature(lazy_type_alias)]

enum Type {
    Variant,
}

fn main() {
    type Alias = Type;
    let _ = Alias::Variant;
}
