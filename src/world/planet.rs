use crate::render::Vertex;

pub fn load_model(path: &str) -> (Vec<Vertex>, Vec<u32>, gltf::image::Data, gltf::image::Data, gltf::image::Data, gltf::image::Data) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new(); // Now implicitly Vec<u32>

    // Use the dynamic path variable!
    let (document, buffers, images) = gltf::import(path).unwrap_or_else(|_| panic!("Failed to load {}", path));

    let mut color_idx = None;
    let mut normal_idx = None;
    let mut mr_idx = None; 
    let mut ao_idx = None; 

    for material in document.materials() {
        if let Some(tex) = material.pbr_metallic_roughness().base_color_texture() { color_idx = Some(tex.texture().source().index()); }
        if let Some(tex) = material.normal_texture() { normal_idx = Some(tex.texture().source().index()); }
        if let Some(tex) = material.pbr_metallic_roughness().metallic_roughness_texture() { mr_idx = Some(tex.texture().source().index()); }
        if let Some(tex) = material.occlusion_texture() { ao_idx = Some(tex.texture().source().index()); }
    }

    let get_image = |index: Option<usize>, fallback: [u8; 4]| -> gltf::image::Data {
        if let Some(i) = index {
            if i < images.len() { return images[i].clone(); }
        }
        gltf::image::Data { pixels: fallback.to_vec(), format: gltf::image::Format::R8G8B8A8, width: 1, height: 1 }
    };

    let diffuse_image = get_image(color_idx, [255, 255, 255, 255]);
    let normal_image = get_image(normal_idx, [128, 128, 255, 255]); 
    let mr_image = get_image(mr_idx, [0, 128, 0, 255]); 
    let ao_image = get_image(ao_idx, [255, 255, 255, 255]); 

    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            let vertex_offset = vertices.len() as u32;

            let positions: Vec<[f32; 3]> = reader.read_positions().expect("Mesh has no positions!").collect();
            let normals: Vec<[f32; 3]> = reader.read_normals().expect("Mesh has no normals!").collect();
            
            let tex_coords: Vec<[f32; 2]> = if let Some(tc) = reader.read_tex_coords(0) { tc.into_f32().collect() } 
            else { vec![[0.0, 0.0]; positions.len()] };

            let tangents: Vec<[f32; 4]> = if let Some(t) = reader.read_tangents() { t.collect() } 
            else { vec![[1.0, 0.0, 0.0, 1.0]; positions.len()] };

            for i in 0..positions.len() {
                vertices.push(Vertex { position: positions[i], normal: normals[i], tex_coords: tex_coords[i], tangent: tangents[i], color: [1.0, 1.0, 1.0] });
            }

            if let Some(indices_iter) = reader.read_indices() {
                // FIXED: Changed to keep it as u32 instead of u16
                for index in indices_iter.into_u32() { indices.push(index + vertex_offset); }
            }
        }
    }

    (vertices, indices, diffuse_image, normal_image, mr_image, ao_image)
}