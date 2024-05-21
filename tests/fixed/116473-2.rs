enum Enum<T> {  SVariant { _v: T }, UVariant }

type AliasFixed = Enum<()>;

macro_rules! test {
    () => (test!(UVariant));
    ($variant:ident) => (if let AliasFixed::$variant::<3> {} = 5 { true } else { false });
}

fn main() {
    test!();
}
