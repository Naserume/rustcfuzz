#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait {
async fn bar(& abc self);
}

impl MyTrait for () {
async fn bar(&self) {}
}

fn main() {}
