union Union {
    val: [u8],
}

fn cast(ptr: *const ()) -> *const Union {
    ptr as _
}
