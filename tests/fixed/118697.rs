struct Foo {
    bar: dyn for<v = Bar2(&NoisyDrop(&u))> Fn(usize),
}
