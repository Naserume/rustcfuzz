trait Trait {
    fn foo(&self) -> u32 { 0 }
}

struct F;

impl Trait for S {
    reuse to_reuse::foo { self }    
    reuse <F as Trait>::foo;
}
