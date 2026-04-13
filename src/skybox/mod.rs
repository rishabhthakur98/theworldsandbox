use glam::Vec3;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CelestialBody {
    pub direction: [f32; 4], // xyz = direction, w = Type (0.0=Point, 1.0=Disc, 2.0=Crescent)
    pub color: [f32; 4],     // rgb = color, a = intensity
    pub params: [f32; 4],    // x = diameter(rad), y = edge_softness, z = crescent_offset, w = unused
}

impl CelestialBody {
    pub fn new_point(azimuth: f32, elevation: f32, color: [f32; 3], intensity: f32, size: f32, glow: f32) -> Self {
        let dir = Self::calc_dir(azimuth, elevation);
        Self {
            direction: [dir.x, dir.y, dir.z, 0.0],
            color: [color[0], color[1], color[2], intensity],
            params: [size.to_radians(), glow, 0.0, 0.0],
        }
    }

    pub fn new_disc(azimuth: f32, elevation: f32, color: [f32; 3], intensity: f32, diameter: f32, softness: f32) -> Self {
        let dir = Self::calc_dir(azimuth, elevation);
        Self {
            direction: [dir.x, dir.y, dir.z, 1.0],
            color: [color[0], color[1], color[2], intensity],
            params: [diameter.to_radians(), softness.to_radians(), 0.0, 0.0],
        }
    }

    pub fn new_crescent(azimuth: f32, elevation: f32, color: [f32; 3], intensity: f32, diameter: f32, softness: f32, phase_offset: f32) -> Self {
        let dir = Self::calc_dir(azimuth, elevation);
        Self {
            direction: [dir.x, dir.y, dir.z, 2.0],
            color: [color[0], color[1], color[2], intensity],
            params: [diameter.to_radians(), softness.to_radians(), phase_offset.to_radians(), 0.0],
        }
    }

    fn calc_dir(azimuth_deg: f32, elevation_deg: f32) -> Vec3 {
        let az = azimuth_deg.to_radians();
        let el = elevation_deg.to_radians();
        Vec3::new(el.cos() * az.sin(), el.sin(), el.cos() * az.cos()).normalize()
    }
}