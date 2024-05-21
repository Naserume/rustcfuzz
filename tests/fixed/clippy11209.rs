#![feature(return_position_impl_trait_in_trait)]

pub trait Application {
    fn setup() -> Result<(), impl std::error::Error>;
}
