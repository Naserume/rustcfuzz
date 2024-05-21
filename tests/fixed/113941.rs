enum Void {}

pub struct Struct([*const (); 0], Void);

pub enum Enum {
    Variant(Struct),
}
