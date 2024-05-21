#![feature(lazy_type_alias)]

use std::io;

pub trait Strategy {}

pub struct Subpaths<S: Strategy> {}

impl<S: Strategy> Subpaths<S> {
    pub fn new() -> io::Result<Subpaths<S>> {}
}
