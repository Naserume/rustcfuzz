pub fn test(test: &u64, temp: &u64) {
    let temp = temp.clone();
    async || {
        test.clone();
        temp.abs_diff(12);
    };
}
