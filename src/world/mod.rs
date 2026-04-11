pub mod planet;

use crate::render::Vertex;

pub fn generate_world() -> (Vec<Vertex>, Vec<u16>, gltf::image::Data) {
    let base_y = 0.0;
    
    // Pass the base_y down and return the vertices, indices, and the texture image
    planet::generate_planet(base_y)
}