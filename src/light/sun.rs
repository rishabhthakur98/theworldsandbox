use glam::Vec3;
use crate::config;

pub struct Sun {
    pub direction: Vec3,
    pub color: [f32; 4],   // RGBA (A = intensity)
    pub ambient: [f32; 4], // RGBA (A = intensity)
}

impl Sun {
    pub fn new() -> Self {
        let az = config::SUN_AZIMUTH.to_radians();
        let el = config::SUN_ELEVATION.to_radians();

        // Calculate the vector the sun is pointing towards
        let dir_x = el.cos() * az.sin();
        let dir_y = -el.sin(); // Negative because it shines down from the sky
        let dir_z = el.cos() * az.cos();

        Self {
            direction: Vec3::new(dir_x, dir_y, dir_z).normalize(),
            color: [
                config::SUN_COLOR[0], 
                config::SUN_COLOR[1], 
                config::SUN_COLOR[2], 
                config::SUN_INTENSITY
            ],
            ambient: [
                config::AMBIENT_COLOR[0], 
                config::AMBIENT_COLOR[1], 
                config::AMBIENT_COLOR[2], 
                config::AMBIENT_INTENSITY
            ],
        }
    }
}