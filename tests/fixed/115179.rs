use std::io::Write;

/// An encoder with its type erased.
pub type DynEncoder<'w, W> = Box<dyn EncodeTo<W> + 'w>;

/// Type trait for a data element encoder to a single known writer type `W`.
pub trait EncodeTo<W: ?Sized> {}

#[derive(Debug)]
pub struct StatefulEncoder<E>(E);

// the type alias below breaks the compiler

pub type DynStatefulEncoder<'w> = StatefulEncoder<DynEncoder<'w, dyn Write>>;
