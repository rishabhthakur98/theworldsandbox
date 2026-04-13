use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub position: [f32; 4],
    pub direction: [f32; 4],
    pub color: [f32; 4],     
    pub params: [f32; 4], 
    pub light_view_proj: [[f32; 4]; 4], 
}

// FIXED: Silences the "associated functions are never used" warnings!
#[allow(dead_code)]
impl Light {
    pub fn new_sun(azimuth_deg: f32, elevation_deg: f32, color: [f32; 3], intensity: f32, cast_shadows: bool) -> Self {
        let az = azimuth_deg.to_radians();
        let el = elevation_deg.to_radians();
        let dir = Vec3::new(el.cos() * az.sin(), -el.sin(), el.cos() * az.cos());

        let sun_virtual_pos = -dir * 100.0;
        let view = Mat4::look_at_rh(sun_virtual_pos, Vec3::ZERO, Vec3::Y);
        let proj = Mat4::orthographic_rh(-50.0, 50.0, -50.0, 50.0, 0.1, 200.0);

        Self {
            position: [0.0, 0.0, 0.0, 0.0], 
            direction: [dir.x, dir.y, dir.z, 0.0],
            color: [color[0], color[1], color[2], intensity],
            params: [if cast_shadows { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0],
            light_view_proj: (proj * view).to_cols_array_2d(),
        }
    }

    pub fn new_spot(pos: [f32; 3], dir: [f32; 3], inner: f32, outer: f32, color: [f32; 3], intensity: f32, radius: f32, cast_shadows: bool) -> Self {
        let direction = Vec3::from(dir).normalize();
        
        let view = Mat4::look_at_rh(Vec3::from(pos), Vec3::from(pos) + direction, Vec3::Y);
        let proj = Mat4::perspective_rh(outer.to_radians() * 2.0, 1.0, 0.1, radius);

        Self {
            position: [pos[0], pos[1], pos[2], 2.0],
            direction: [direction.x, direction.y, direction.z, inner.to_radians().cos()],
            color: [color[0], color[1], color[2], intensity],
            params: [if cast_shadows { 1.0 } else { 0.0 }, radius, outer.to_radians().cos(), 0.0],
            light_view_proj: (proj * view).to_cols_array_2d(),
        }
    }

    pub fn new_point(pos: [f32; 3], color: [f32; 3], intensity: f32, radius: f32, cast_shadows: bool) -> Self {
        let view = Mat4::look_at_rh(Vec3::from(pos), Vec3::from(pos) + Vec3::NEG_Y, Vec3::Z);
        let proj = Mat4::perspective_rh(120.0_f32.to_radians(), 1.0, 0.1, radius);

        Self {
            position: [pos[0], pos[1], pos[2], 1.0],
            direction: [0.0, -1.0, 0.0, 0.0],
            color: [color[0], color[1], color[2], intensity],
            params: [if cast_shadows { 1.0 } else { 0.0 }, radius, 0.0, 0.0],
            light_view_proj: (proj * view).to_cols_array_2d(),
        }
    }
}