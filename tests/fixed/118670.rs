

trait Table: Send {
    type Key;
}

struct Walker<T: Table> {
    start: T,
}

impl<T: Table> std::iter::Iterator for Walker<T> {
    type Item = Result<<T as Table>::Key, Box<dyn std::error::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
