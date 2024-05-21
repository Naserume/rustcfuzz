// rustc +nightly -Zmir-opt-level=0 -Zmir-enable-passes=+Inline

fn main() {
    let func = || 123u8;
    func();
}
