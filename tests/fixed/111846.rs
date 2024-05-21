#![feature(return_position_impl_trait_in_trait, return_type_notation)]

trait IntFactory {
    fn stream(&self) -> impl Iterator<Item = i32>;
}
trait SendIntFactory: IntFactory<stream(): Send> + Send {}
