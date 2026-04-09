mod app;
mod camera;
mod render;
mod world;

use app::App;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    
    let mut app = App { state: None };
    event_loop.run_app(&mut app).unwrap();
}