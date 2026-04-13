pub mod planet;

use crate::render::Vertex;
use crate::engine::entity::Entity;
use crate::light::Light;
use glam::Vec3;

pub struct ModelData {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub diffuse: gltf::image::Data,
    pub normal: gltf::image::Data,
    pub mr: gltf::image::Data,
    pub ao: gltf::image::Data,
    pub disp: gltf::image::Data, 
}

pub fn generate_world() -> (Vec<ModelData>, Vec<Light>) {
    
    // 1. YOUR LEVEL GEOMETRY
    let level = vec![
        Entity::new("assets/tunnel.glb", [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
    ];

    // 2. YOUR DYNAMIC LIGHTS
    let lights = vec![
        // Sun shining diagonally
        Light::new_sun(20.0, 90.0, [1.0, 0.95, 0.9], 3.0, true), 
        // Light::new_sun(100.0, 90.0, [100.0, 0.95, 0.9], 3.0, true), 
        
        // A Warm torch deep inside the tunnel
        // Light::new_point([0.0, 2.0, 10.0], [1.0, 0.5, 0.2], 15.0, 20.0, false),
        
        // A Street Lamp pointing straight down at the entrance
        // Light::new_spot(
        //     [0.0, 10.0, -5.0],  // Positioned high up
        //     [0.0, -1.0, 0.0],   // Pointing straight down
        //     12.5,               // Inner Cone Angle
        //     17.5,               // Outer Cone Angle (smooth fade)
        //     [1.0, 0.9, 0.6],    // Color
        //     20.0,               // Intensity
        //     25.0,               // Max Radius
        //     true               // Cast shadows
        // ),
    ];

    let mut models = Vec::new();

    for entity in level {
        let (mut verts, inds, diff, norm, mr, ao, disp) = planet::load_model(&entity.glb_path);
        let model_matrix = entity.get_model_matrix();
        let normal_matrix = model_matrix.inverse().transpose(); 

        for v in verts.iter_mut() {
            let pos = model_matrix.transform_point3(Vec3::from(v.position));
            v.position = pos.into();
            let n = normal_matrix.transform_vector3(Vec3::from(v.normal)).normalize();
            v.normal = n.into();
            let t_dir = normal_matrix.transform_vector3(Vec3::new(v.tangent[0], v.tangent[1], v.tangent[2])).normalize();
            v.tangent = [t_dir.x, t_dir.y, t_dir.z, v.tangent[3]];
        }

        models.push(ModelData { vertices: verts, indices: inds, diffuse: diff, normal: norm, mr: mr, ao: ao, disp: disp });
    }

    (models, lights)
}