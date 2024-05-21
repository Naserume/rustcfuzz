trait Trait<const module_path: Trait = bar> {
    async fn handle<F>(slf: &F)
where
    F: Fn(&()) -> &mut (dyn Future<Output = ()> + Unpin),
{
    (slf)(&()).await;
}
}

fn main() {}
