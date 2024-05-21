union W { s: dyn Iterator<Item = Missing> }

static ONCE: W = todo!();
