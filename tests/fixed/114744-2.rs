pub fn accept(_: impl Trait<K: Copy>) {}

pub trait Trait {
    const K: i32;
}
