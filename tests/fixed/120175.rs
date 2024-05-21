#![feature(extern_types)]

#[link(name = "bar", import_name_type = "decorated", kind = "raw-dylib")]
extern "C" {
    pub type CrossCrate;
}

fn main() {}
