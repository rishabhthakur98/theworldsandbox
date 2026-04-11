pub mod planet;

use crate::render::Vertex;

// Now returning FOUR images (Diffuse, Normal, Roughness, AO)
pub fn generate_world() -> (Vec<Vertex>, Vec<u16>, gltf::image::Data, gltf::image::Data, gltf::image::Data, gltf::image::Data) {
    let base_y = 0.0;
    planet::generate_planet(base_y)
}