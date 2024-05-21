use run_make_support::{htmldocck, rustdoc, tmp_dir};

fn main() {
    let out_dir = tmp_dir().join("rustdoc");
    rustdoc().input("src/lib.rs").crate_name("foobar").crate_type("lib").output(&out_dir).run();
    assert!(htmldocck().arg(out_dir).arg("src/lib.rs").status().unwrap().success());
}