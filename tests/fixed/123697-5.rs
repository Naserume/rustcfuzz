pub fn test(test: &u64, temp: &u64) {
    let other = test.clone();
    async || {
        temp.clone();
        other.abs_diff(12);
    };
}
