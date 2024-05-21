#![feature(const_closures)]
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

/// This won't work since doc tests are in a different crate
/// ```rust
/// const _:() = playground::test();
/// ```
pub const fn test() {
    let cl = const || {};
    cl();
}
const _: () = {
    // Works in this crate
    test();
};
