// revisions: sized clone

fn rpit() -> impl Sized {}

fn same_output<Out>(_: impl Fn() -> Out, _: impl Fn() -> Out) {}

pub fn foo() -> impl Sized {
    same_output(rpit, foo);
    same_output(foo, rpit);
    rpit()
}
