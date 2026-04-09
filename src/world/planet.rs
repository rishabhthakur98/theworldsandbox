use crate::render::Vertex;
use std::f32::consts::PI;

pub fn generate_planet(radius: f32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

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

            vertices.push(Vertex { 
                position: [x, y, z], 
                normal: [x / radius, y / radius, z / radius], 
                color: planet_color 
            });
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

    (vertices, indices)
}