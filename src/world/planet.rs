use crate::render::Vertex;

pub fn generate_planet(_base_y: f32) -> (Vec<Vertex>, Vec<u16>, gltf::image::Data) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 1. Load the binary glTF file 
    let (document, buffers, mut images) = gltf::import("assets/planet.glb").expect("Failed to load planet.glb");

    // ---------------------------------------------------------
    // NEW: Find the actual Base Color (Albedo) image index!
    // ---------------------------------------------------------
    let mut color_image_index = 0; // Fallback to 0
    for material in document.materials() {
        // Look for the PBR base color texture
        if let Some(texture_info) = material.pbr_metallic_roughness().base_color_texture() {
            color_image_index = texture_info.texture().source().index();
            break; // We found it, stop looking
        }
    }

    // Extract the correct image based on the material data
    let image_data = images.remove(color_image_index);
    // ---------------------------------------------------------

    // 2. Loop through the file to find the 3D meshes
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let vertex_offset = vertices.len() as u32;

            let positions: Vec<[f32; 3]> = reader.read_positions().expect("Mesh has no positions!").collect();
            let normals: Vec<[f32; 3]> = reader.read_normals().expect("Mesh has no normals!").collect();
            
            let tex_coords: Vec<[f32; 2]> = if let Some(tc) = reader.read_tex_coords(0) {
                tc.into_f32().collect()
            } else {
                vec![[0.0, 0.0]; positions.len()] 
            };

            for i in 0..positions.len() {
                vertices.push(Vertex {
                    position: positions[i],
                    normal: normals[i],
                    tex_coords: tex_coords[i], 
                    color: [1.0, 1.0, 1.0], 
                });
            }

            if let Some(indices_iter) = reader.read_indices() {
                for index in indices_iter.into_u32() {
                    indices.push((index + vertex_offset) as u16);
                }
            }
        }
    }

    (vertices, indices, image_data)
}