mod events;
mod font_atlas;
mod gui;
mod rain;
mod renderer;

use std::sync::Arc;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Matrix Digital Rain")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .unwrap();

    let window = Arc::new(window);

    let mut app = pollster::block_on(gui::App::new(window.clone()));

    event_loop
        .run(|event, target| {
            app.handle_event(&event, target);
        })
        .unwrap();
}
