// Replaced CameraUniform with a GlobalUniform containing light info
struct GlobalUniform {
    view_proj: mat4x4<f32>,
    sun_dir: vec4<f32>,     
    sun_color: vec4<f32>,   
    ambient_color: vec4<f32>, 
};
@group(0) @binding(0)
var<uniform> global: GlobalUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = global.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    out.color = model.color; 
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(in.normal);
    
    // Reverse the sun direction because lighting math requires calculating 
    // the angle from the surface *towards* the light source.
    let light_dir = normalize(-global.sun_dir.xyz);
    
    // 1. Calculate Diffuse Light (Direct sunlight)
    let diffuse_strength = max(dot(normal, light_dir), 0.0);
    let diffuse_color = global.sun_color.xyz * global.sun_color.a * diffuse_strength;
    
    // 2. Calculate Ambient Light (The glow in the shadows)
    let ambient = global.ambient_color.xyz * global.ambient_color.a;
    
    // Combine and apply
    let final_light = ambient + diffuse_color;
    let final_color = in.color * final_light;
    
    return vec4<f32>(final_color, 1.0);
}