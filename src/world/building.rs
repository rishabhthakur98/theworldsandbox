use crate::render::Vertex;

pub fn generate_building(base_y: f32, start_idx: u16) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // The new massive scale
    let b_width = 50.0;
    let b_depth = 50.0;
    let b_height = 500.0;
    let b_color = [0.0, 0.0, 1.0]; // Blue

    let hw = b_width / 2.0;
    let hd = b_depth / 2.0;
    let top_y = base_y + b_height;

    // 24 Vertices for hard-edged faces
    let build_verts = [
        // Front (+Z)
        ([-hw, base_y,  hd], [0.0, 0.0, 1.0]), ([ hw, base_y,  hd], [0.0, 0.0, 1.0]), ([ hw,  top_y,  hd], [0.0, 0.0, 1.0]), ([-hw,  top_y,  hd], [0.0, 0.0, 1.0]),
        // Back (-Z)
        ([ hw, base_y, -hd], [0.0, 0.0, -1.0]), ([-hw, base_y, -hd], [0.0, 0.0, -1.0]), ([-hw,  top_y, -hd], [0.0, 0.0, -1.0]), ([ hw,  top_y, -hd], [0.0, 0.0, -1.0]),
        // Left (-X)
        ([-hw, base_y, -hd], [-1.0, 0.0, 0.0]), ([-hw, base_y,  hd], [-1.0, 0.0, 0.0]), ([-hw,  top_y,  hd], [-1.0, 0.0, 0.0]), ([-hw,  top_y, -hd], [-1.0, 0.0, 0.0]),
        // Right (+X)
        ([ hw, base_y,  hd], [1.0, 0.0, 0.0]), ([ hw, base_y, -hd], [1.0, 0.0, 0.0]), ([ hw,  top_y, -hd], [1.0, 0.0, 0.0]), ([ hw,  top_y,  hd], [1.0, 0.0, 0.0]),
        // Top (+Y)
        ([-hw,  top_y,  hd], [0.0, 1.0, 0.0]), ([ hw,  top_y,  hd], [0.0, 1.0, 0.0]), ([ hw,  top_y, -hd], [0.0, 1.0, 0.0]), ([-hw,  top_y, -hd], [0.0, 1.0, 0.0]),
        // Bottom (-Y)
        ([-hw, base_y, -hd], [0.0, -1.0, 0.0]), ([ hw, base_y, -hd], [0.0, -1.0, 0.0]), ([ hw, base_y,  hd], [0.0, -1.0, 0.0]), ([-hw, base_y,  hd], [0.0, -1.0, 0.0]),
    ];

    for (pos, norm) in build_verts.iter() {
        vertices.push(Vertex { position: *pos, normal: *norm, color: b_color });
    }

    let build_indices = [
        0, 1, 2,  2, 3, 0,       // Front
        4, 5, 6,  6, 7, 4,       // Back
        8, 9, 10, 10, 11, 8,     // Left
        12, 13, 14, 14, 15, 12,  // Right
        16, 17, 18, 18, 19, 16,  // Top
        20, 21, 22, 22, 23, 20,  // Bottom
    ];

    for i in build_indices.iter() {
        indices.push(start_idx + i);
    }

    (vertices, indices)
}