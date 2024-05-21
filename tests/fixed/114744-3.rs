pub fn accept<T>()
where
    T: Trait,
    T::K: Copy,
{
}

pub trait Trait {
    const K: i32;
}
