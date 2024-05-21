trait Foo {}

impl<T, #[rustc_host] const host: bool = true> Foo for T {}
