#![feature(trivial_bounds)]

#[derive(Debug)]
struct TwoStrs(str, str)
where
    str: Sized;
