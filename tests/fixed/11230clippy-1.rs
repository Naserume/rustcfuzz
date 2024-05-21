trait Helper<'a>: Iterator<Item = fn()> {}

fn x(w: &mut dyn for<'a> Helper<'a>) {
    w.collect::<Vec<_>>().is_empty();
}
