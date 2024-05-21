pub enum Constraint<T: ?Sized> {
  Eq(Value),
  Gt(Value),
   // ...
}

impl<T: ?Sized> Constraint<T> {
  pub fn mask_chunk(&self, chunk: &dyn Chunk) -> BitVec {
    fn eq(ord: Option<Ordering>) -> bool { /* ... */ }
    fn gt(ord: Option<Ordering>) -> bool { /* ... */ }

    let (value, cmp) = match self {
        Self::Eq(value) => (value, eq),
        Self::Gt(value) => (value, gt),
    };

    if let Some(chunk) = chunk.downcast_ref::<SomeType>() {
      chunk
        .iter()
        .map(|row| row.partial_cmp(value))
        .map(cmp)
        .collect()
    } else {
        /* ... */
    }
  }
}
