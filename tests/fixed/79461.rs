
trait IAmATrait {
    type Item;
    fn function(&self) -> Self::Item;
}

struct IAmAnObject(usize);

impl IAmATrait for IAmAnObject {
    type Item = _;
    fn function(&self) -> Self::Item {
        self.0
    }
}

fn main() {}
