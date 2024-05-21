async fn empty() {}

async fn meow(a: [u8; 0xffff_ffff_ffff_ffff]) {
    empty().await;
    dbg!(a);
}
