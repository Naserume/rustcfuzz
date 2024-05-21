trait X {
    fn line_stream<'a, Repr>() -> Self::LineStreamFut<{ async {} }, Repr>;
}

struct Y;
