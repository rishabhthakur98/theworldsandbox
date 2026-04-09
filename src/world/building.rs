use crate::render::Vertex;

pub fn generate_building(base_y: f32, start_idx: u16) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let b_width = 50.0;
    let b_depth = 50.0;
    let b_height = 500.0;
    let b_color = [0.0, 0.0, 1.0]; 

    let hw = b_width / 2.0;
    let hd = b_depth / 2.0;
    let top_y = base_y + b_height;

    let build_verts = [
        ([-hw, base_y,  hd], [0.0, 0.0, 1.0]), ([ hw, base_y,  hd], [0.0, 0.0, 1.0]), ([ hw,  top_y,  hd], [0.0, 0.0, 1.0]), ([-hw,  top_y,  hd], [0.0, 0.0, 1.0]),
        ([ hw, base_y, -hd], [0.0, 0.0, -1.0]), ([-hw, base_y, -hd], [0.0, 0.0, -1.0]), ([-hw,  top_y, -hd], [0.0, 0.0, -1.0]), ([ hw,  top_y, -hd], [0.0, 0.0, -1.0]),
        ([-hw, base_y, -hd], [-1.0, 0.0, 0.0]), ([-hw, base_y,  hd], [-1.0, 0.0, 0.0]), ([-hw,  top_y,  hd], [-1.0, 0.0, 0.0]), ([-hw,  top_y, -hd], [-1.0, 0.0, 0.0]),
        ([ hw, base_y,  hd], [1.0, 0.0, 0.0]), ([ hw, base_y, -hd], [1.0, 0.0, 0.0]), ([ hw,  top_y, -hd], [1.0, 0.0, 0.0]), ([ hw,  top_y,  hd], [1.0, 0.0, 0.0]),
        ([-hw,  top_y,  hd], [0.0, 1.0, 0.0]), ([ hw,  top_y,  hd], [0.0, 1.0, 0.0]), ([ hw,  top_y, -hd], [0.0, 1.0, 0.0]), ([-hw,  top_y, -hd], [0.0, 1.0, 0.0]),
        ([-hw, base_y, -hd], [0.0, -1.0, 0.0]), ([ hw, base_y, -hd], [0.0, -1.0, 0.0]), ([ hw, base_y,  hd], [0.0, -1.0, 0.0]), ([-hw, base_y,  hd], [0.0, -1.0, 0.0]),
    ];

    for (pos, norm) in build_verts.iter() {
        vertices.push(Vertex { position: *pos, normal: *norm, color: b_color });
    }

    let build_indices = [
        0, 1, 2,  2, 3, 0,       
        4, 5, 6,  6, 7, 4,       
        8, 9, 10, 10, 11, 8,     
        12, 13, 14, 14, 15, 12,  
        16, 17, 18, 18, 19, 16,  
        20, 21, 22, 22, 23, 20,  
    ];

    for i in build_indices.iter() {
        indices.push(start_idx + i);
    }

    (vertices, indices)
}