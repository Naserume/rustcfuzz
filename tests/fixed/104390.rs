fn main() {}

fn a() -> impl Sized { & 2E }

fn b() -> impl Sized { && 2E }

fn c() -> impl Sized { &'a 2E }

fn d() -> impl Sized { &'static 2E }
