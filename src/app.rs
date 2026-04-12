use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{CursorGrabMode, Window, WindowId, Fullscreen},
};

use crate::config;
use crate::control::{KEY_FORWARD, KEY_BACKWARD, KEY_LEFT, KEY_RIGHT};
use crate::engine::state::State;

pub struct App {
    pub state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_none() {
            let mut window_builder = Window::default_attributes()
                .with_title(config::WINDOW_TITLE)
                .with_inner_size(LogicalSize::new(config::WINDOW_WIDTH as f64, config::WINDOW_HEIGHT as f64));

            if config::IS_FULLSCREEN { window_builder = window_builder.with_fullscreen(Some(Fullscreen::Borderless(None))); }

            let window = Arc::new(event_loop.create_window(window_builder).unwrap());
            let _ = window.set_cursor_grab(CursorGrabMode::Confined);
            window.set_cursor_visible(false);
            self.state = Some(pollster::block_on(State::new(window)));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => state.resize(physical_size),
            WindowEvent::KeyboardInput { event: KeyEvent { state: key_state, physical_key, .. }, .. } => {
                let pressed = key_state == ElementState::Pressed;
                if let PhysicalKey::Code(code) = physical_key {
                    if code == winit::keyboard::KeyCode::Escape { event_loop.exit(); }
                    match code {
                        KEY_FORWARD => state.input.forward = pressed,
                        KEY_BACKWARD => state.input.backward = pressed,
                        KEY_LEFT => state.input.left = pressed,
                        KEY_RIGHT => state.input.right = pressed,
                        _ => {}
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                state.update();
                state.render();
            }
            _ => {}
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: winit::event::DeviceId, event: DeviceEvent) {
        if let Some(state) = &mut self.state {
            if let DeviceEvent::MouseMotion { delta } = event { state.camera.handle_mouse(delta.0, delta.1); }
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(state) = &mut self.state { state.window.request_redraw(); }
    }
}