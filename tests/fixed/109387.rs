#![feature(type_alias_impl_trait)]

trait Trait<T> {
    type Assoc: Default;
}

type Ty<T> = impl Trait<T>;
    
impl<T> Trait<T> for T
where
    <Ty<T> as Trait<T>>::Assoc: Default,
{
    type Assoc = <Ty<T> as Trait<T>>::Assoc;
}

#[allow(unused)]
fn define<T>(x: T) -> Ty<T> { x }

fn main() {
    let _ = <() as Trait<()>>::Assoc::default();
}
