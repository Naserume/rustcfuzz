#![feature(non_lifetime_binders)]
#![crate_type = "lib"]

trait Trait<Input> {
    type Output;
}

async fn walk2<F: 'a>(filter: F)
where
    for<F> F: Trait<&'a u32> + 'a,
{}

fn main() {}
