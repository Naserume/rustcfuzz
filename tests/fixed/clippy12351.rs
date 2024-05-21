fn main() {
    let a = xm1::S { ..panic!() };
    //~^ ERROR failed to resolve: could not find `imp` in `sys` [E0433]
    //~^^ ERROR module `sys` is private [E0603]
}
