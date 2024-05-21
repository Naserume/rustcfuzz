#![feature(async_fn_in_trait)]

pub(crate) trait Inbox<M> {
    async fn next(self) -> M;
}

pub(crate) trait Actor: Sized {
    type Message;

    async fn on_mount(self, _: impl Inbox<Self::Message>);
}

impl<'a> Actor for () {
    type Message = &'a ();
    async fn on_mount(self, _: impl Inbox<&'a ()>) {}
}

fn main(){}
