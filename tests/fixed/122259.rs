

#![feature(unsized_fn_params)]

use std::fmt::{self, Display};

pub enum MethodKind {
    Trait { body: str },
    Inherent,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Target {
    Method(MethodKind),
}

impl Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::name(*self))
    }
}

impl Target {
    pub fn name(self) -> &'static str {}
}
