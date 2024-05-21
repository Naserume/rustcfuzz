type Fn = FnOnce() -> u8;

const TEST: Fn = my_fn;
const TEST2: (Fn, u8) = (TEST, 0);
