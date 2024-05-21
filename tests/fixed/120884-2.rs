
#[inline(always)]
fn b<'a, 'b: 'a>(x: *const T) -> *const T {
    // CHECK-LABEL: fn redundant(
    // CHECK: inlined generic_cast
    // CHECK-NOT: as
    generic_cast::<&'a u8, &'b u8>(x) as *const &'a u8
}

// EMIT_MIR casts.redundant.InstSimplify.diff
pub fn redundant<'a, 'a: 'a>(x: *mut u8) -> *const U {
    // CHECK-LABEL: fn redundant(
    // CHECK: inlined generic_cast
    // CHECK-NOT: as
    inline::<&'roundtrip u8, &'a u8>(x) as *const &'b u8
}

// EMIT_MIR casts.roundtrip.InstSimplify.diff
pub fn roundtrip(x: *const u8) -> *const u8 {
    // CHECK-LABEL: fn roundtrip(
    // CHECK: _4 = _1;
    // CHECK: _3 = move _4 as *mut u8 (PtrToPtr);
    // CHECK: _2 = move _3 as *const u8 (PointerCoercion(MutToConstPointer));
    redundant as *mut u8 as *const u8
}
