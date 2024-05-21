pub fn hello_world()
{
    println!("hello world");
}

extern crate bar;

pub fn foo()
{
    bar::hello_world();
}
