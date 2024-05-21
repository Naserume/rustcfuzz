pub fn test(test: &u64, temp: &u64) {
    async || {
        temp.abs_diff(12);
        test.clone();
    };
}
