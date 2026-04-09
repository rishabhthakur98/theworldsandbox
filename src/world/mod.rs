pub mod building; // Declare the new module so Rust compiles it

use crate::render::Vertex;
use std::f32::consts::PI;

pub fn generate_world() -> (Vec<Vertex>, Vec<u16>) {
    let radius = 390.0;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // --- 1. GENERATE PLANET (White) ---
    let rings = 8;
    let sectors = 12;
    let planet_color = [1.0, 1.0, 1.0];

    for r in 0..=rings {
        let v = r as f32 / rings as f32;
        let phi = v * PI;
        for s in 0..=sectors {
            let u = s as f32 / sectors as f32;
            let theta = u * PI * 2.0;

            let x = radius * phi.sin() * theta.cos();
            let y = radius * phi.cos();
            let z = radius * phi.sin() * theta.sin();

            vertices.push(Vertex { position: [x, y, z], normal: [x / radius, y / radius, z / radius], color: planet_color });
        }
    }

    for r in 0..rings {
        for s in 0..sectors {
            let first = (r * (sectors + 1)) + s;
            let second = first + sectors + 1;
            indices.extend_from_slice(&[
                first as u16, second as u16, (first + 1) as u16,
                second as u16, (second + 1) as u16, (first + 1) as u16,
            ]);
        }
    }

    // --- 2. GENERATE BUILDING (Blue) ---
    // Pass in the planet's radius so it sits on top, and the current amount of vertices 
    // so the index offsets are correct.
    let start_idx = vertices.len() as u16;
    let (b_verts, b_indices) = building::generate_building(radius, start_idx);

    // Merge the building into the main world lists
    vertices.extend(b_verts);
    indices.extend(b_indices);

    (vertices, indices)
}