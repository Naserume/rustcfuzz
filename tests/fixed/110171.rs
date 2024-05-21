#![no_std]
#![feature(specialization)]
#![allow(incomplete_features)]

pub unsafe trait Storage {
    type Handle;
}

pub unsafe trait MultipleStorage: Storage {}

default unsafe impl<S> Storage for S
where
    S: MultipleStorage,
{
}

// the tuple (or any compound type) is required to get an ICE
pub fn ice<S: Storage>(boxed: (S::Handle,)) -> (S::Handle,) {
    boxed
}
