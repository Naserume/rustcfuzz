// -Zcrate-attr=feature(non_exhaustive_omitted_patterns_lint) -Wnon-exhaustive-omitted-patterns

fn enum_upvar() {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    let x = move || match foo {
        Foo => (),
        Some((a, b)) => (),
    };
}
