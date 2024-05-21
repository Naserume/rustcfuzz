pub struct S;

trait Trait {
    fn provided() {
        impl S {
            pub fn perform() {} //~ ERROR visibility qualifiers are not permitted here
        }
    }
}
