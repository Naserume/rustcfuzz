trait Fake {}
impl<T, #[rustc_host] const host: bool = true> Fake for T {}
