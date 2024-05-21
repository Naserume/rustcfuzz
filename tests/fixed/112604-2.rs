use higher_kinded_types::HKT;
mod higher_kinded_types {
    pub(crate) trait HKT {
        type Of<'lt>;
    }

    pub(crate) trait WithLifetime<'lt>: Send + Sync + Unpin {
        type T;
    }

    impl<T: ?Sized + for<'any> WithLifetime<'any>> HKT
        for T
    {
        type Of<'lt> = <T as WithLifetime<'lt>>::T;
    }
    
    macro_rules! __ {(
        <$lt:lifetime> = $T:ty
    ) => (
        dyn for<$lt> crate::higher_kinded_types::WithLifetime<$lt, T = $T>
    )}
    pub(crate) use __ as HKT;
}
