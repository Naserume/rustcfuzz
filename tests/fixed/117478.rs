#![feature(const_trait_impl, effects)]

#[const_trait]
trait X {
    const fn x();
}
