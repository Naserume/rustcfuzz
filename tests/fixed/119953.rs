pub fn fun(number: usize, decision: bool) {
    for _ in 0..2 {
        for _ in 0..number {
            let iter: Box<dyn Iterator<Item = ()>> = if decision {
                Box::new(std::iter::empty())
            } else {
                Box::new(std::iter::once(()))
            };
            for _ in iter {}
        }
    }
}
