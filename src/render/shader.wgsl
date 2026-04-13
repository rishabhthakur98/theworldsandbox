struct Light {
    position: vec4<f32>,  
    direction: vec4<f32>, 
    color: vec4<f32>,     
    params: vec4<f32>,    
    light_view_proj: mat4x4<f32>, 
};

struct GlobalUniform {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    ambient_color: vec4<f32>,
    active_lights: u32,
    displacement_scale: f32, 
    use_pom: u32, 
    _pad: u32,
    lights: array<Light, 16>, 
};
@group(0) @binding(0) var<uniform> global: GlobalUniform;
@group(0) @binding(1) var t_shadow: texture_depth_2d_array; 
@group(0) @binding(2) var s_shadow: sampler_comparison;     

@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_sampler: sampler; 
@group(1) @binding(2) var t_normal: texture_2d<f32>;
@group(1) @binding(3) var t_mr: texture_2d<f32>; 
@group(1) @binding(4) var t_ao: texture_2d<f32>; 
@group(1) @binding(5) var t_disp: texture_2d<f32>; 

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
    @location(4) world_position: vec3<f32>, 
    @location(5) tangent_view_pos: vec3<f32>, 
    @location(6) tangent_frag_pos: vec3<f32>, 
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
    
    let N = normalize(model.normal);
    let T_raw = normalize(model.tangent.xyz);
    let T = normalize(T_raw - dot(T_raw, N) * N);
    let B = cross(N, T) * model.tangent.w;
    let TBN = mat3x3<f32>(T, B, N);
    let invTBN = transpose(TBN);

    out.tangent_view_pos = invTBN * global.camera_pos.xyz;
    out.tangent_frag_pos = invTBN * model.position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.tex_coords;
    if (global.use_pom == 1u) {
        let view_dir_ts = normalize(in.tangent_view_pos - in.tangent_frag_pos);
        let depth_val = 1.0 - textureSample(t_disp, s_sampler, in.tex_coords).r;
        let p = view_dir_ts.xy * (depth_val * global.displacement_scale); 
        uv = in.tex_coords - p;
    }

    let tex_color = textureSample(t_diffuse, s_sampler, uv);
    let normal_map_val = textureSample(t_normal, s_sampler, uv).xyz;
    let mr_val = textureSample(t_mr, s_sampler, uv);
    let ao_val = textureSample(t_ao, s_sampler, uv).r;

    let roughness = max(mr_val.g, 0.05); 
    let base_reflectivity = 0.04;

    var final_normal = normalize(in.normal);
    let tangent_length = length(in.tangent.xyz);
    if (tangent_length > 0.01) {
        let tangent_normal = normal_map_val * 2.0 - vec3<f32>(1.0, 1.0, 1.0);
        let T = normalize(in.tangent.xyz);
        let T_ortho = normalize(T - dot(T, final_normal) * final_normal);
        let B = cross(final_normal, T_ortho) * in.tangent.w;
        let TBN = mat3x3<f32>(T_ortho, B, final_normal);
        final_normal = normalize(TBN * tangent_normal);
    }

    let view_dir = normalize(global.camera_pos.xyz - in.world_position);
    var total_light = global.ambient_color.xyz * global.ambient_color.a * ao_val;

    for (var i = 0u; i < global.active_lights; i = i + 1u) {
        let light = global.lights[i];
        var l_dir = vec3<f32>(0.0);
        var attenuation = 1.0;

        if (light.position.w == 0.0) { // SUN
            l_dir = normalize(-light.direction.xyz);
        } else if (light.position.w == 1.0) { // POINT
            let light_vec = light.position.xyz - in.world_position;
            let distance = length(light_vec);
            if (distance > light.params.y) { continue; } 
            l_dir = normalize(light_vec);
            attenuation = clamp(1.0 - (distance / light.params.y), 0.0, 1.0);
        } else if (light.position.w == 2.0) { // SPOT
            let light_vec = light.position.xyz - in.world_position;
            let distance = length(light_vec);
            if (distance > light.params.y) { continue; } 
            l_dir = normalize(light_vec);
            let theta = dot(l_dir, normalize(-light.direction.xyz));
            let epsilon = light.direction.w - light.params.z; 
            let spot_intensity = clamp((theta - light.params.z) / epsilon, 0.0, 1.0);
            attenuation = clamp(1.0 - (distance / light.params.y), 0.0, 1.0) * spot_intensity;
        }

        // --- SHADOW CALCULATION ---
        if (light.params.x > 0.0) {
            let shadow_pos = light.light_view_proj * vec4<f32>(in.world_position, 1.0);
            var shadow_coords = shadow_pos.xyz / shadow_pos.w; 
            
            shadow_coords.x = shadow_coords.x * 0.5 + 0.5;
            shadow_coords.y = shadow_coords.y * -0.5 + 0.5;
            
            // Tightened bias to safely account for the massive 500m Sun camera box!
            let current_depth = shadow_coords.z;
            let bias = max(0.001 * (1.0 - dot(final_normal, l_dir)), 0.0001); 

            if (shadow_coords.x >= 0.0 && shadow_coords.x <= 1.0 && 
                shadow_coords.y >= 0.0 && shadow_coords.y <= 1.0 && 
                current_depth >= 0.0 && current_depth <= 1.0) {
                
                let shadow_factor = textureSampleCompare(t_shadow, s_shadow, shadow_coords.xy, i32(i), current_depth - bias);
                attenuation *= shadow_factor;
            } else if (light.position.w == 2.0) {
                // Completely erase light that escapes outside the Spotlight camera bounds
                attenuation = 0.0;
            }
        }

        let half_dir = normalize(l_dir + view_dir);
        let radiance = light.color.xyz * light.color.w * attenuation;
        let n_dot_l = max(dot(final_normal, l_dir), 0.0);
        let diffuse = radiance * n_dot_l;
        let shininess = 2.0 / (pow(roughness, 4.0)) - 2.0;
        let n_dot_h = max(dot(final_normal, half_dir), 0.0);
        let spec_strength = pow(n_dot_h, max(shininess, 1.0)) * ((shininess + 2.0) / 8.0);
        let specular = radiance * spec_strength * (1.0 - roughness) * base_reflectivity;

        total_light += (diffuse * ao_val) + specular;
    }

    let final_color = (in.color * tex_color.xyz) * total_light;
    return vec4<f32>(final_color, tex_color.a);
}