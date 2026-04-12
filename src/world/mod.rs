pub mod planet;

use crate::render::Vertex;
use crate::engine::entity::Entity;
use glam::Vec3;

// NEW: A struct to hold each object's data separately!
pub struct ModelData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub diffuse: gltf::image::Data,
    pub normal: gltf::image::Data,
    pub mr: gltf::image::Data,
    pub ao: gltf::image::Data,
}

pub fn generate_world() -> Vec<ModelData> {
    
    // ---------------------------------------------------------
    // LEVEL DESIGNER: Add as many objects here as you want!
    // ---------------------------------------------------------
    let level = vec![
        Entity::new("assets/planet.glb", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
        Entity::new("assets/tunnel.glb", [0.0, 5.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
    ];
    // ---------------------------------------------------------

    let mut models = Vec::new();

    // Loop through every object you spawned
    for entity in level {
        // Load the specific file
        let (mut verts, inds, diff, norm, mr, ao) = planet::load_model(&entity.glb_path);
        
        let model_matrix = entity.get_model_matrix();
        let normal_matrix = model_matrix.inverse().transpose(); // Required to fix lighting when rotated!

        // Apply your custom Position, Rotation, and Scale math to the vertices!
        for v in verts.iter_mut() {
            // Move Position
            let pos = model_matrix.transform_point3(Vec3::from(v.position));
            v.position = pos.into();

            // Rotate Normals
            let n = normal_matrix.transform_vector3(Vec3::from(v.normal)).normalize();
            v.normal = n.into();

            // Rotate Tangents
            let t_dir = normal_matrix.transform_vector3(Vec3::new(v.tangent[0], v.tangent[1], v.tangent[2])).normalize();
            v.tangent = [t_dir.x, t_dir.y, t_dir.z, v.tangent[3]];
        }

        // Push the independent model to the list
        models.push(ModelData {
            vertices: verts,
            indices: inds,
            diffuse: diff,
            normal: norm,
            mr: mr,
            ao: ao,
        });
    }

    models
}