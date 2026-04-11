struct GlobalUniform {
    view_proj: mat4x4<f32>,
    sun_dir: vec4<f32>,     
    sun_color: vec4<f32>,   
    ambient_color: vec4<f32>, 
};
@group(0) @binding(0) var<uniform> global: GlobalUniform;

// NEW: Texture and Sampler Bindings
@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>, 
    @location(3) color: vec3<f32>,      
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>, 
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = global.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    out.color = model.color; 
    out.tex_coords = model.tex_coords;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the color from the image using our UV coordinates
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    let normal = normalize(in.normal);
    let light_dir = normalize(-global.sun_dir.xyz);
    
    let diffuse_strength = max(dot(normal, light_dir), 0.0);
    let diffuse_color = global.sun_color.xyz * global.sun_color.a * diffuse_strength;
    let ambient = global.ambient_color.xyz * global.ambient_color.a;
    
    let final_light = ambient + diffuse_color;
    
    // Multiply the base color, the texture color, and the light together
    let final_color = in.color * tex_color.xyz * final_light;
    
    return vec4<f32>(final_color, 1.0);
}