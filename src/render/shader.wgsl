struct GlobalUniform {
    view_proj: mat4x4<f32>,
    sun_dir: vec4<f32>,     
    sun_color: vec4<f32>,   
    ambient_color: vec4<f32>, 
};
@group(0) @binding(0) var<uniform> global: GlobalUniform;

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;
// NEW: Normal map bindings
@group(1) @binding(2) var t_normal: texture_2d<f32>;
@group(1) @binding(3) var s_normal: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>, 
    @location(3) tangent: vec4<f32>,
    @location(4) color: vec3<f32>,      
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>, 
    @location(3) tangent: vec4<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = global.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    out.color = model.color; 
    out.tex_coords = model.tex_coords;
    out.tangent = model.tangent;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let normal_map_val = textureSample(t_normal, s_normal, in.tex_coords).xyz;

    // Convert normal map colors [0 to 1] into physical directions [-1 to 1]
    let tangent_normal = normal_map_val * 2.0 - vec3<f32>(1.0);

    // Calculate TBN Matrix (Tangent, Bitangent, Normal)
    let N = normalize(in.normal);
    let T = normalize(in.tangent.xyz);
    let T_ortho = normalize(T - dot(T, N) * N);
    let B = cross(N, T_ortho) * in.tangent.w;
    let TBN = mat3x3<f32>(T_ortho, B, N);
    
    // Bend the flat normal using the image data
    let final_normal = normalize(TBN * tangent_normal);

    let light_dir = normalize(-global.sun_dir.xyz);
    
    // Apply lighting against our new bumped normal
    let diffuse_strength = max(dot(final_normal, light_dir), 0.0);
    let diffuse_color = global.sun_color.xyz * global.sun_color.a * diffuse_strength;
    let ambient = global.ambient_color.xyz * global.ambient_color.a;
    
    let final_light = ambient + diffuse_color;
    let final_color = in.color * tex_color.xyz * final_light;
    
    return vec4<f32>(final_color, 1.0);
}