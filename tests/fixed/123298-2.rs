#![feature(auto_traits)]

pub struct Outer<'a>(fn(&'a ()));

impl U for for<'a> fn(&'a ()) {}

pub auto trait U {}
