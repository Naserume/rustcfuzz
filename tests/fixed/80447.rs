extern crate proc_macro;
use proc_macro::TokenStream;
#[proc_macro_attribute]
pub fn mac(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    input
}

pub trait Crash {
    #[mac::mac]
    fn one(s: () {
    }

    fn two();
}

fn main () {}
