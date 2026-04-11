use crate::render::Vertex;

pub fn generate_planet(_base_y: f32) -> (Vec<Vertex>, Vec<u16>, gltf::image::Data, gltf::image::Data, gltf::image::Data, gltf::image::Data) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let (document, buffers, images) = gltf::import("assets/planet.glb").expect("Failed to load planet.glb");

    let mut color_idx = None;
    let mut normal_idx = None;
    let mut mr_idx = None; // Metallic-Roughness map
    let mut ao_idx = None; // Ambient Occlusion map

    // 1. Hunt down the indices for all 4 PBR textures
    for material in document.materials() {
        if let Some(tex) = material.pbr_metallic_roughness().base_color_texture() { color_idx = Some(tex.texture().source().index()); }
        if let Some(tex) = material.normal_texture() { normal_idx = Some(tex.texture().source().index()); }
        if let Some(tex) = material.pbr_metallic_roughness().metallic_roughness_texture() { mr_idx = Some(tex.texture().source().index()); }
        if let Some(tex) = material.occlusion_texture() { ao_idx = Some(tex.texture().source().index()); }
    }

    // 2. Helper to safely grab the image, or generate a dummy if it doesn't exist
    let get_image = |index: Option<usize>, fallback: [u8; 4]| -> gltf::image::Data {
        if let Some(i) = index {
            if i < images.len() { return images[i].clone(); }
        }
        gltf::image::Data { pixels: fallback.to_vec(), format: gltf::image::Format::R8G8B8A8, width: 1, height: 1 }
    };

    // Extract all 4 maps with safe fallbacks
    let diffuse_image = get_image(color_idx, [255, 255, 255, 255]);
    let normal_image = get_image(normal_idx, [128, 128, 255, 255]); // Flat normal fallback
    let mr_image = get_image(mr_idx, [0, 128, 0, 255]); // Green channel = Roughness = 0.5
    let ao_image = get_image(ao_idx, [255, 255, 255, 255]); // Red channel = AO = 1.0 (No shadow)

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
                for index in indices_iter.into_u32() { indices.push((index + vertex_offset) as u16); }
            }
        }
    }

    (vertices, indices, diffuse_image, normal_image, mr_image, ao_image)
}