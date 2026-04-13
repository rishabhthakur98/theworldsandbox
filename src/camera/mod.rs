use crate::config;
use glam::{Mat4, Vec3};
use crate::control::InputState;

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vec3::from_array(config::START_POS),
            yaw: config::START_YAW.to_radians(),
            pitch: config::START_PITCH.to_radians(),
        }
    }

    pub fn update_position(&mut self, dt: f32, input: &InputState) {
        let speed = config::CAMERA_SPEED * dt; 
        
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        
        let forward = Vec3::new(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize();
        let right = forward.cross(Vec3::Y).normalize();

        if input.forward { self.pos += forward * speed; }
        if input.backward { self.pos -= forward * speed; }
        if input.left { self.pos -= right * speed; }
        if input.right { self.pos += right * speed; }
    }

    pub fn handle_mouse(&mut self, delta_x: f64, delta_y: f64) {
        self.yaw += delta_x as f32 * config::MOUSE_SENSITIVITY;
        self.pitch -= delta_y as f32 * config::MOUSE_SENSITIVITY;
        let limit = config::PITCH_LIMIT.to_radians();
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn get_view_proj(&self, width: f32, height: f32) -> [[f32; 4]; 4] {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        let view_forward = Vec3::new(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize();

        let view = Mat4::look_to_rh(self.pos, view_forward, Vec3::Y);
        let proj = Mat4::perspective_rh(f32::to_radians(45.0), width / height, 0.1, 10000.0);
        
        // Applied the WebGPU fix to the main camera!
        (crate::light::OPENGL_TO_WGPU_MATRIX * proj * view).to_cols_array_2d()
    }
}