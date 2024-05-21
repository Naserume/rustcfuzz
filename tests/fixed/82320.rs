extern crate proc_macro;

#[proc_macro]
pub fn p(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    panic!()
}

extern crate p;
p::p! {}
