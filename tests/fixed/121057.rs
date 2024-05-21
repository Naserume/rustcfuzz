// requires winit version 0.28.7
use winit::event_loop::EventLoop;



fn main() {
    let event_loop = EventLoop::new();

    event_loop.run(move |event, _, _| {
        event.to_static();

        print!("{:?}", event);
    });
}
