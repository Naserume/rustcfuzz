fn foo()
where
    for<const N: usize = {
        || {
            assert!(u1 == u2);
        };
    }> ():,
{
}
