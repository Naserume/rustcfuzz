#![feature(type_alias_impl_trait)]

fn rpit() -> impl Sized {}

pub type Tait = impl Sized;

pub fn foo() -> Box<dyn std::future::Future<Output = Tait>> {
    Box::new(async {
        let _out: Tait = rpit();
        _out
    })
}

pub fn main() {}
