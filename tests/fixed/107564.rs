trait Trait<'a> {
    type Assoc;
}

fn foo<T: for<'a> Trait<'a>>() -> for<'a> fn(<T as Trait<'a>>::Assoc) {
    todo!()
}

fn bar<T: for<'a> Trait<'a>>() {
    let _: for<'a> fn(<_ as Trait<'a>>::Assoc) = foo::<T>();
}
