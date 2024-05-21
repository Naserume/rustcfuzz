pub enum HiddenType {}

pub trait HiddenTrait {}

pub enum MyLibType {}


impl From<HiddenType> for MyLibType {
    fn from(it: HiddenType) -> MyLibType {
        match it {}
    }
}

pub struct T<T>(T);

impl From<T<T<T<T<HiddenType>>>>> for MyLibType {
    fn from(it: T<T<T<T<HiddenType>>>>) -> MyLibType {
        todo!()
    }
}
