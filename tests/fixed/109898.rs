trait Trait {
    async fn method() {}
}

fn foo<T: Trait<method(i32): Send>>() {}

fn main() {}
