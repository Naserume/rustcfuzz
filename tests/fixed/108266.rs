#![allow(deprecated)]
pub struct Encoder<'a> {}

mod tests {
    #[derive(RustcEncodable)]
    struct Struct { }
}
