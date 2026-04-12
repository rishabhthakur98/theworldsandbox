pub mod planet;

use crate::render::Vertex;
use crate::engine::entity::Entity;
use glam::Vec3;

// FIXED: Changed Vec<u16> to Vec<u32> in return type
pub fn generate_world() -> (Vec<Vertex>, Vec<u32>, gltf::image::Data, gltf::image::Data, gltf::image::Data, gltf::image::Data) {
    
    // ---------------------------------------------------------
    // LEVEL DESIGNER: Add as many objects here as you want!
    // ---------------------------------------------------------
    let level = vec![
        // Tunnel 1 at the center
        Entity::new("assets/planet.glb", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
        
        // Tunnel 2, connected to the end of Tunnel 1 (Z = -20m)
        Entity::new("assets/tunnel.glb", [0.0, 0.0, -20.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
    ];
    // ---------------------------------------------------------

    let mut all_vertices = Vec::new();
    let mut all_indices = Vec::new(); // Implicit Vec<u32>
    let mut first_textures = None;

    // Loop through every object you spawned
    for entity in level {
        // Load the specific file
        let (mut verts, mut inds, diff, norm, mr, ao) = planet::load_model(&entity.glb_path);
        
        // Note: For now, we assume all objects share the same texture to keep the GPU fast
        if first_textures.is_none() {
            first_textures = Some((diff, norm, mr, ao));
        }

        let model_matrix = entity.get_model_matrix();
        let normal_matrix = model_matrix.inverse().transpose(); // Required to fix lighting when rotated!
        
        // FIXED: Using u32 so we don't overflow when adding multiple models!
        let index_offset = all_vertices.len() as u32; 

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

        for i in inds.iter_mut() {
            *i += index_offset;
        }

        all_vertices.extend(verts);
        all_indices.extend(inds);
    }

    let (diff, norm, mr, ao) = first_textures.unwrap();
    (all_vertices, all_indices, diff, norm, mr, ao)
}