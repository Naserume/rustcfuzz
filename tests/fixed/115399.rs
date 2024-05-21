#[repr(transparent)]
#[derive(Default)]
struct Wrapper2<T>((), T);

type T = (i8, i16, f32);

extern "C" fn test(x: Wrapper2<T>) {}

fn main() {
    test(Default::default());
}
