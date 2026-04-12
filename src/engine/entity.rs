use glam::{Mat4, Quat, Vec3};

pub struct Entity {
    pub glb_path: String,
    pub position: [f32; 3],
    pub rotation: [f32; 3], // Pitch (X), Yaw (Y), Roll (Z) in degrees
    pub scale: [f32; 3],
}

impl Entity {
    // A clean function to spawn an object
    pub fn new(path: &str, pos: [f32; 3], rot_degrees: [f32; 3], scale: [f32; 3]) -> Self {
        Self {
            glb_path: path.to_string(),
            position: pos,
            // Math requires radians, so we convert from easy-to-read degrees!
            rotation: [
                rot_degrees[0].to_radians(), 
                rot_degrees[1].to_radians(), 
                rot_degrees[2].to_radians()
            ],
            scale,
        }
    }

    // This generates the 4x4 Transformation Matrix
    pub fn get_model_matrix(&self) -> Mat4 {
        let translation = Mat4::from_translation(Vec3::from(self.position));
        
        let rotation = Mat4::from_quat(Quat::from_euler(
            glam::EulerRot::XYZ, 
            self.rotation[0], self.rotation[1], self.rotation[2]
        ));
        
        let scale = Mat4::from_scale(Vec3::from(self.scale));

        // Multiply T * R * S
        translation * rotation * scale
    }
}