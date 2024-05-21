macro_rules! inst {
    ($name:ident ( $exe:expr;)) => {
        Instruction {
            execute: $exe,
        }
    };
}

struct Instruction {
    execute: fn(),
}

fn main() {
    let _i = inst!(
        INST ( || { Err(()) }; )
    );
}
