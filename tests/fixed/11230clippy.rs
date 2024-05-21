fn main() {
  const A: &[for<'a> fn(&'a ())] = &[];
  for v in A.iter() {}
}
