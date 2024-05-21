struct NoDerive(i32);

impl PartialEq for NoDerive {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

impl Eq for NoDerive {}

#[derive(PartialEq, Eq)]
struct WrapInline<'a>(&'a &'a NoDerive);

const WRAP_DOUBLY_INDIRECT_INLINE: &&WrapInline = &&WrapInline(&&NoDerive(0));

fn main() {
    match WRAP_DOUBLY_INDIRECT_INLINE {
        WRAP_DOUBLY_INDIRECT_INLINE | _ => {}
    }
}
