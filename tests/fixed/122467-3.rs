pub struct GenericStruct<const M: usize, T>(T);

impl<T> GenericStruct<0, T> {
    pub fn new(thing: T) -> GenericStruct<1, T> {
        Self {0: thing}
    }
}
