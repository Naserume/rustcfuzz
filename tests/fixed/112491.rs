trait Trait {
    const ASSOC: fn(&'static u32);
}
impl Trait for () {
    const ASSOC: Fn(i32) -> _ = |_| ();
}

fn main() {
    let _ = <() as Trait>::ASSOC;
}
