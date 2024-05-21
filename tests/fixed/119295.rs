extern "C" {
    pub fn lint_me(
        x: Bar<
            S<
                {
                    type B<b> = impl ;
                },
            >,
        >,
    );
}
