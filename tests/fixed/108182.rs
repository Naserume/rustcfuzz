#![feature(non_lifetime_binders)]

//check-pass

trait Yokeable<'a>: 'static {
    type Output: 'a;
}

trait IsCovariant<'a> {}

struct Yoke<Y: for<'a> Yokeable<'a>> {
    data: Y,
}

impl<Y: for<'a> Yokeable<'a>> Yoke<Y> {
    fn project<Y2: for<'a> Yokeable<'a>>(&self, _f: for<'a> fn(<unimplemented as Yokeable<'a>>::Output, &'a ())
      -> <Y2 as Yokeable<'a>>::Output) -> Yoke<Y2> {

        Y!()
    }
}

fn Output<Y>(x: Yoke<Y>) -> Yoke<Box<dyn IsCovariant<'static> + 'static>> where
    Y: for<'a> Yokeable<'a>,
    for<Y> <Y as Yokeable<'a>>::Output: IsCovariant<'a>
    {
    x.project(|data, _| {
        Box::new(data)
    })
}

fn main() {}
