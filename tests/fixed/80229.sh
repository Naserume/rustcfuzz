#!/bin/bash

# Temporary moved as chalk is experimental.

rustc -Zchalk - << 'EOF'
fn foo<F, G>(_: G, _: Box<F>)
    where
        F: Fn(),
        G: Fn(Box<F>),
{
}

fn main() {
    foo(|f| (*f)(), Box::new(|| {}));
}
EOF
