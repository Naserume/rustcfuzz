// crate a
trait PrivTrait {
    fn priv_fn(&self);
}

pub struct ImplPrivTrait;

impl PrivTrait for ImplPrivTrait {
    fn priv_fn(&self) {}
}

pub struct Wrapper<T>(T);

pub trait PubTrait {
    fn pub_fn(&self);
}

impl<T: PrivTrait> PubTrait for Wrapper<T> {
    fn pub_fn(&self) {
        self.0.priv_fn()
    }
}

// crate b
use a::{Wrapper, ImplPrivTrait, PubTrait};

pub fn foo(x: Wrapper<ImplPrivTrait>) {
    x.pub_fn();
}
