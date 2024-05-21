trait Role<C> {
    type I;
}

struct H;

impl<C: Fn(&())> Role<C> for H {
    type I = ();
}

struct Machine<C, R: Role<C>> {
    _i: R::I,
}

impl<C, R: Role<C>> Drop for Machine<C, R> {
    fn drop(&mut self) {}
}

fn accept<C: Fn(&()), R: Role<C>>(_callback: C) -> Machine<C, R> {
    todo!()
}

fn main() {
    let callback = |_| {};
    accept::<_, H>(callback);
}
