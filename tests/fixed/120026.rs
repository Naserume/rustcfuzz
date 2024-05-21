struct GenericStruct<const MY_NUM: usize> {
    val: i64,
}

impl<const T: u128> From<GenericStruct<T>> for GenericStruct<{}> {}

#![feature(with_negative_coherence)]
