#![feature(generic_const_exprs)]

type A<T> = for<const N: () = { T::B }> fn();
