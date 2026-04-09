use winit::keyboard::KeyCode;

// Key Bindings Configuration
pub const KEY_FORWARD: KeyCode = KeyCode::KeyW;
pub const KEY_BACKWARD: KeyCode = KeyCode::KeyS;
pub const KEY_LEFT: KeyCode = KeyCode::KeyA;
pub const KEY_RIGHT: KeyCode = KeyCode::KeyD;

pub struct InputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            forward: false,
            backward: false,
            left: false,
            right: false,
        }
    }
}