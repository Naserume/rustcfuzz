// Box<T> can be any generic; Option<T> or Result<T, [anything]> also work

fn a<T>(x: Box<T>) {}

// This must be async; return type is left out for minimal-ness, but it could be modified to return a Box<()> and the error would still exist (but it'd return a future still, and awaiting that future makes the bug disappear)

async fn b() {}

fn main() {
    // Bug disappears if b() is awaited
    
    a(b());
}
