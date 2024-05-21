union W<T> { s: dyn Iterator<Item = Missing> }

static ONCE: W<()> = todo!();
