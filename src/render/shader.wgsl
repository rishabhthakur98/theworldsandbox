struct GlobalUniform {
    view_proj: mat4x4<f32>,
    sun_dir: vec4<f32>,     
    sun_color: vec4<f32>,   
    ambient_color: vec4<f32>, 
    camera_pos: vec4<f32>, // NEW: Needed for specular reflections!
};
@group(0) @binding(0) var<uniform> global: GlobalUniform;

// TEXTURE BINDINGS
@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_sampler: sampler; // One shared sampler for all 4 textures!
@group(1) @binding(2) var t_normal: texture_2d<f32>;
@group(1) @binding(3) var t_mr: texture_2d<f32>; // Metallic-Roughness
@group(1) @binding(4) var t_ao: texture_2d<f32>; // Ambient Occlusion

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
    @location(4) world_position: vec3<f32>, // NEW: Pass exact 3D location to fragment
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = global.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    out.color = model.color; 
    out.tex_coords = model.tex_coords;
    out.tangent = model.tangent;
    out.world_position = model.position; 
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Sample all 4 textures
    let tex_color = textureSample(t_diffuse, s_sampler, in.tex_coords);
    let normal_map_val = textureSample(t_normal, s_sampler, in.tex_coords).xyz;
    let mr_val = textureSample(t_mr, s_sampler, in.tex_coords);
    let ao_val = textureSample(t_ao, s_sampler, in.tex_coords).r;

    // GLTF Standard: Green channel is Roughness. AO map uses the Red channel.
    let roughness = mr_val.g; 
    let ao = ao_val;

    // 2. Normal Bending (TBN Matrix)
    let tangent_normal = normal_map_val * 2.0 - vec3<f32>(1.0);
    let N = normalize(in.normal);
    let T = normalize(in.tangent.xyz);
    let T_ortho = normalize(T - dot(T, N) * N);
    let B = cross(N, T_ortho) * in.tangent.w;
    let TBN = mat3x3<f32>(T_ortho, B, N);
    let final_normal = normalize(TBN * tangent_normal);

    // 3. Lighting Directions
    let light_dir = normalize(-global.sun_dir.xyz);
    let view_dir = normalize(global.camera_pos.xyz - in.world_position);
    let half_dir = normalize(light_dir + view_dir); // Used for reflections

    // 4. Calculate PBR Components
    let diffuse_strength = max(dot(final_normal, light_dir), 0.0);
    let diffuse_color = global.sun_color.xyz * global.sun_color.a * diffuse_strength;
    
    // AMBIENT OCCLUSION: Multiply ambient light by the AO map value
    let ambient = global.ambient_color.xyz * global.ambient_color.a * ao;
    
    // SPECULAR REFLECTION (Shininess): Driven by the Roughness map
    let min_roughness = max(roughness, 0.05); // Prevent divide by zero
    let shininess = 2.0 / (pow(min_roughness, 4.0)) - 2.0;
    let spec_angle = max(dot(final_normal, half_dir), 0.0);
    let specular_strength = pow(spec_angle, max(shininess, 1.0));
    
    // Rougher surfaces have weaker, spread-out highlights. Smooth surfaces have tight, bright ones.
    let specular_color = global.sun_color.xyz * global.sun_color.a * specular_strength * (1.0 - roughness);

    // 5. Combine (Note: Specular light sits ON TOP of diffuse color, it is not multiplied by it!)
    let diffuse_ambient = ambient + diffuse_color;
    let final_color = (in.color * tex_color.xyz * diffuse_ambient) + specular_color;
    
    return vec4<f32>(final_color, 1.0);
}