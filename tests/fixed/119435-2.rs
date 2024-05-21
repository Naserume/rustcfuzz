use std::ops::Index;

pub fn function(arg: impl Index<()>) {
    42 & arg[()];
}
