use crate::render::Vertex;

pub fn generate_planet(_base_y: f32) -> (Vec<Vertex>, Vec<u16>, gltf::image::Data, gltf::image::Data) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let (document, buffers, images) = gltf::import("assets/planet.glb").expect("Failed to load planet.glb");

    // Find the correct image indices by looking at the Material
    let mut color_index = 0;
    let mut normal_index = 0;

    for material in document.materials() {
        if let Some(tex) = material.pbr_metallic_roughness().base_color_texture() {
            color_index = tex.texture().source().index();
        }
        if let Some(tex) = material.normal_texture() {
            normal_index = tex.texture().source().index();
        }
    }

    // Clone the images out so we can pass both of them to the GPU
    let diffuse_image = images[color_index].clone();
    let normal_image = images[normal_index].clone();

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

            // NEW: Read Tangents (Required for Normal Mapping)
            let tangents: Vec<[f32; 4]> = if let Some(t) = reader.read_tangents() {
                t.collect()
            } else {
                // Fallback dummy tangent if the file didn't export them
                vec![[1.0, 0.0, 0.0, 1.0]; positions.len()] 
            };

            for i in 0..positions.len() {
                vertices.push(Vertex {
                    position: positions[i],
                    normal: normals[i],
                    tex_coords: tex_coords[i], 
                    tangent: tangents[i], // Add tangent to vertex
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

    (vertices, indices, diffuse_image, normal_image)
}