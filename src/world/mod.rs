pub mod building;
pub mod planet;

use crate::render::Vertex;

pub fn generate_world() -> (Vec<Vertex>, Vec<u16>) {
    let radius = 390.0;
    
    // 1. Get the planet
    let (mut vertices, mut indices) = planet::generate_planet(radius);

    // 2. Get the building and append it
    let start_idx = vertices.len() as u16;
    let (b_verts, b_indices) = building::generate_building(radius, start_idx);

    vertices.extend(b_verts);
    indices.extend(b_indices);

    (vertices, indices)
}