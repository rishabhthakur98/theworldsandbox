use crate::render::Vertex;

// Note: We use an underscore _radius because the size is now fully controlled by Blender!
pub fn generate_planet(_radius: f32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // 1. Load the binary glTF file from the assets folder
    let (document, buffers, _) = gltf::import("assets/planet.glb").expect("Failed to load planet.glb");

    // 2. Loop through the file to find the 3D meshes
    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            
            // Keep track of our starting index in case your file has multiple pieces
            let vertex_offset = vertices.len() as u32;

            // 3. Read positions and normals from the file
            let positions: Vec<[f32; 3]> = reader.read_positions().expect("Mesh has no positions!").collect();
            let normals: Vec<[f32; 3]> = reader.read_normals().expect("Mesh has no normals!").collect();

            // 4. Combine them into our game's Vertex format
            for i in 0..positions.len() {
                vertices.push(Vertex {
                    position: positions[i],
                    normal: normals[i],
                    color: [1.0, 1.0, 1.0], // Forcing it to be pure white!
                });
            }

            // 5. Read the triangle indices
            if let Some(indices_iter) = reader.read_indices() {
                for index in indices_iter.into_u32() {
                    // Convert to u16 for our wgpu index buffer
                    indices.push((index + vertex_offset) as u16);
                }
            }
        }
    }

    (vertices, indices)
}