fn main() {
    let f = | _ , y: &u32 , z | ();
    thing(f);
    //~^ ERROR mismatched types
    //~^^ ERROR mismatched types
    let f = | x, y: _  , z: u32 | ();
    thing(f);
    //~^ ERROR mismatched types
    //~^^ ERROR mismatched types
    //~^^^ ERROR implementation of `FnOnce` is not general enough
    //~^^^^ ERROR implementation of `FnOnce` is not general enough
}

fn thing(&u32, &u32, u32) {
    let f = | x, y: _  , z: u32 | ();
    thing(y);
    //~^ ERROR mismatched types
    //~^^ ERROR mismatched types
    let f = | _ , y: &u32 , z | ();
    f(f);
    //~^ ERROR mismatched types
    //~^^ ERROR mismatched types
    //~^^^ ERROR implementation of `FnOnce` is not general enough
    //~^^^ ERROR implementation of `FnOnce` is not general enough
}
