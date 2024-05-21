#![feature(non_lifetime_binders)]

fn main()  {
    for<main> || -> & main  {};
}
