use std::marker::PhantomData;

// The variance of both generics don't seem to matter.
// I tried with *mut E, *const E, and fn(E). Every one gives the same result.
struct Example<E, FakeParam>(PhantomData<(E, FakeParam)>);

 // This can also have a lifetime of any variance, it doesn't appear to matter.
struct NoLifetime;
// Likewise, the variance of `'a` here also doesn't seem to matter.
struct Immutable<'a>(PhantomData<&'a ()>);

// The E: 'a is part of what causes the ICE. if it is just `E`, the compiler correctly
// finds the `Copy` bound missing on `Example<E, Mutable<'a>>`
impl<'a, E: 'a> Copy for Example<E, Immutable<'a>> {}
impl<'a, E: 'a> Clone for Example<E, Immutable<'a>> {
    fn clone(&self) -> Self { *self }
}

impl<E, FakeParam> Example<E, FakeParam> {
    // there is no ICE if this (correctly) takes `&self`.
    unsafe fn change<NewFakeParam>(self) -> Example<E, NewFakeParam> {
        Example(PhantomData)
    }
}

impl<E> Example<E, NoLifetime> {
    fn the_ice(&mut self) -> Example<E, Immutable<'_>> {
        unsafe { self.change() }
    }
}
