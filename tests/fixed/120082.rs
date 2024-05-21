trait ThreeCellFragment {
    fn ext_cells<'a>(&'a self) -> Box<dyn Foo> + 'a {
        self.ext_adjacent_cells()
    }

    fn ext_adjacent_cells<'a>(&'a self) -> Box<dyn Foo> + 'a;
}
fn main() {}
