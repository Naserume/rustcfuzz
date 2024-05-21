use std::collections::HashSet;

fn main() {
    let maybe_hash_set: Option<HashSet<()>> = None;
    for _ in maybe_hash_set.as_ref().unwrap_or(&HashSet::new()) {}
}
