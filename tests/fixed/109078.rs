struct Foo(Option<Box<Fn(&EContext)>>);
const TRANSMUTED_U32: u32 = { std::mem::transmute(Foo(582)) };
