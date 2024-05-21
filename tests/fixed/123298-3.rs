pub struct Outer<'a>(Local<fn(&'a ())>);

struct Local<T>(T);

impl std::marker::Unpin for Local<for<'x> fn(&'x ())> {}
