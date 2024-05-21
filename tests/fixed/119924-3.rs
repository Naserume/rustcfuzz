pub struct S;

#[const_trait]
trait Trait {
    fn required();
}

impl const Trait for () {
    fn required() {
        impl S {
            pub fn perform() {} //~ ERROR visibility qualifiers are not permitted here
        }
    }
}
