// Test to ensure that there is no ICE when normalizing a projection
// which is invalid (from <https://github.com/rust-lang/rust/pull/106938>).

#![crate_type = "lib"]

trait Identity {
    type Identity;
}
trait NotImplemented {}

impl<T: NotImplemented> Identity for T {
    type Identity = Self;
}

type Foo = u8;

union D { //~ ERROR union with unnamed fields must have `#[repr(C)]` representation
          //~^ NOTE union `D` defined here
    _: Foo, //~ NOTE unnamed field defined here
}
