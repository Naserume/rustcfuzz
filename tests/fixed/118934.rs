pub fn b() {
    #[cfg(target_feature = "cpu")]
    a
    #[cfg(not(target_feature = "cpu"))]
}
