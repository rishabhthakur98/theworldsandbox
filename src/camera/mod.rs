use glam::{Mat4, Vec3};

pub struct InputState {
    pub w: bool, pub a: bool, pub s: bool, pub d: bool,
}

pub struct Camera {
    pub pos: Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [[f32; 4]; 4],
}

impl Camera {
    pub fn update_position(&mut self, dt: f32, input: &InputState) {
        let speed = 100.0 * dt; 
        let forward = Vec3::new(self.yaw.cos(), 0.0, self.yaw.sin()).normalize();
        let right = forward.cross(Vec3::Y).normalize();

        if input.w { self.pos += forward * speed; }
        if input.s { self.pos -= forward * speed; }
        if input.a { self.pos -= right * speed; }
        if input.d { self.pos += right * speed; }
    }

    pub fn handle_mouse(&mut self, delta_x: f64, delta_y: f64) {
        let sensitivity = 0.002;
        self.yaw += delta_x as f32 * sensitivity;
        self.pitch -= delta_y as f32 * sensitivity;
        let limit = 89.0_f32.to_radians();
        self.pitch = self.pitch.clamp(-limit, limit);
    }

    pub fn update_uniform(&self, width: f32, height: f32, uniform: &mut CameraUniform) {
        let (yaw_sin, yaw_cos) = self.yaw.sin_cos();
        let (pitch_sin, pitch_cos) = self.pitch.sin_cos();
        let view_forward = Vec3::new(yaw_cos * pitch_cos, pitch_sin, yaw_sin * pitch_cos).normalize();

        let view = Mat4::look_to_rh(self.pos, view_forward, Vec3::Y);
        let proj = Mat4::perspective_rh(f32::to_radians(45.0), width / height, 0.1, 10000.0);
        
        uniform.view_proj = (proj * view).to_cols_array_2d();
    }
}