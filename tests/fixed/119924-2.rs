pub struct S;
struct Expr<const N: u32>;

trait Trait {
    fn required(_: Expr<{
        impl S {
            pub fn perform() {} //~ ERROR visibility qualifiers are not permitted here
        }
        0
    }>);
}
