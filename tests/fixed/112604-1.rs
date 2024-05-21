trait FnRet {
    type Ret;
}
impl<T> FnRet for fn() -> T {
    type Ret = T;
}
type Never = <fn() -> ! as FnRet>::Ret;
mod higher_kinded_types {
    use with_lifetime::WithLifetime;
    pub(crate) mod with_lifetime {
        pub(crate) trait WithLifetime<'lt>: Send + Sync + Unpin {
            type T;
        }
        impl<'lt, T: ?Sized + WithLifetime<'lt>> WithLifetime<'lt>
            for crate::higher_kinded_types::__private::Hkt<T>
        {
            type T = T::T;
        }
    }
    pub(crate) trait Hkt: Send + Sync + Unpin + seal::Sealed {
        type __<'lt>;
    }
    mod seal {
        pub(crate) trait Sealed {}
        impl<T: ?Sized> Sealed for super::__private::Hkt<T> {}
    }
    impl<T: ?Sized> Hkt for T
    where
        Self: for<'any> WithLifetime<'any> + seal::Sealed,
    {
        type __<'lt> = <Self as WithLifetime<'lt>>::T;
    }
    pub(crate) mod __private {
        use crate::Never;
        pub(crate) struct Hkt<T: ?Sized>(::core::marker::PhantomData<T>, Never);
    }
}
use higher_kinded_types::Hkt;
use std::{marker::PhantomData, str::CharIndices};
pub(crate) trait DynParser {
    type Output: Hkt;
    fn dyn_parse<'a>(&self) -> Result<'a, <Self::Output as Hkt>::__<'a>>;
}
impl<T> DynParser for T
where
    T: Parser,
{
    type Output = crate::higher_kinded_types::__private::Hkt<
        dyn for<'a> crate::higher_kinded_types::with_lifetime::WithLifetime<'a, T = T::Output<'a>>,
    >;
    fn dyn_parse<'a>(&self) -> Result<'a, <Self::Output as Hkt>::__<'a>> {
        loop {}
    }
}
pub trait Parser {
    type Output<'a>;
    fn parse<'a>(&self) -> Result<'a, Self::Output<'a>>;
}
impl<Output: Hkt> Parser for dyn '_ + DynParser<Output = Output> {
    type Output<'a> = Output::__<'a>;
    fn parse<'a>(&self) -> Result<'a, Self::Output<'a>> {
        loop {}
    }
}
impl<T> Parser for Box<T>
where
    T: ?Sized + Parser,
{
    type Output<'a> = T::Output<'a>;
    fn parse<'a>(&self) -> Result<'a, Self::Output<'a>> {
        loop {}
    }
}
pub(crate) type Result<'a, T, E = ()> = ::std::result::Result<(CharIndices<'a>, T), E>;
pub(crate) const fn always<T>(value: T) -> impl for<'a> Parser<Output<'a> = T> {
    struct Always<T> {
        value: T,
    }
    impl<T> Parser for Always<T> {
        type Output<'a> = T;
        fn parse<'a>(&self) -> Result<'a, Self::Output<'a>> {
            loop {}
        }
    }
    Always { value }
}
pub(crate) fn lazy<T, F>(_: F) -> impl for<'a> Parser<Output<'a> = T::Output<'a>>
where
    F: FnOnce() -> T,
    T: Parser,
{
    struct Lazy<F, T> {
        inner: PhantomData<T>,
        inner2: PhantomData<F>,
    }
    impl<F, T> Parser for Lazy<F, T>
    where
        F: FnOnce() -> T,
        T: Parser,
    {
        type Output<'a> = T::Output<'a>;
        fn parse<'a>(&self) -> Result<'a, Self::Output<'a>> {
            loop {}
        }
    }
    Lazy {
        inner: PhantomData,
        inner2: PhantomData::<F>,
    }
}
pub(crate) fn uncycle<'a, T>(
    parser: T,
) -> Box<
    dyn 'a
        + DynParser<
            Output = crate::higher_kinded_types::__private::Hkt<
                dyn for<'b> crate::higher_kinded_types::with_lifetime::WithLifetime<
                    'b,
                    T = T::Output<'b>,
                >,
            >,
        >,
>
where
    T: Parser + 'a,
{
    Box::new(parser)
}
/// vvv HERE vvv
pub fn foo() -> impl for<'a> Parser<Output<'a> = ()> {
    lazy(|| uncycle(always(())))
}
