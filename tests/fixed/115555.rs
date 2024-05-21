#![feature(generic_const_exprs)]
fn foo<'a>()
    where [(); {let _: &'a ();}]== [(); {let _: &'a ();}]
{
    //code
}
