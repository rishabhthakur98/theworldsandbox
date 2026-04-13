struct CelestialBody {
    direction: vec4<f32>, 
    color: vec4<f32>,     
    params: vec4<f32>,    
};

struct SkyboxUniform {
    view_proj_inv: mat4x4<f32>,
    horizon_color: vec4<f32>,
    zenith_color: vec4<f32>,
    settings: vec4<u32>, // x = use_panorama, y = num_bodies
    bodies: array<CelestialBody, 8>, 
};
@group(0) @binding(0) var<uniform> sky: SkyboxUniform;

// Optional Panorama Texture
@group(0) @binding(1) var t_pano: texture_2d<f32>;
@group(0) @binding(2) var s_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_ray: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    // Generate a massive full-screen triangle without any vertex buffers!
    let uv = vec2<f32>(f32((vertex_index << 1u) & 2u), f32(vertex_index & 2u));
    let clip_pos = vec4<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 1.0, 1.0); // Z=1.0 puts it behind everything
    
    var out: VertexOutput;
    out.clip_position = clip_pos;
    
    // Un-project the screen pixel into a 3D ray pointing into the world
    let unprojected = sky.view_proj_inv * clip_pos;
    out.world_ray = unprojected.xyz / unprojected.w;
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ray = normalize(in.world_ray);
    var final_color = vec4<f32>(0.0);

    // 1. PANORAMA OR GRADIENT
    if (sky.settings.x == 1u) {
        // Spherical mapping for an equirectangular image
        let pi = 3.14159265;
        let u = atan2(ray.z, ray.x) / (2.0 * pi) + 0.5;
        let v = asin(ray.y) / pi + 0.5;
        final_color = textureSample(t_pano, s_sampler, vec2<f32>(u, 1.0 - v));
    } else {
        // Atmospheric Shift
        let t = clamp(ray.y, 0.0, 1.0); // 0 at horizon, 1 at top
        final_color = mix(sky.horizon_color, sky.zenith_color, t);
    }

    // 2. DRAW CELESTIAL BODIES OVERLAY
    for (var i = 0u; i < sky.settings.y; i = i + 1u) {
        let body = sky.bodies[i];
        let dir = normalize(body.direction.xyz);
        let angle = acos(clamp(dot(ray, dir), -1.0, 1.0));
        let radius = body.params.x;
        let softness = body.params.y;

        if (body.direction.w == 0.0) { // Point / Star
            let intensity = pow(clamp(1.0 - (angle / radius), 0.0, 1.0), softness);
            final_color += body.color * intensity * body.color.w;
            
        } else if (body.direction.w == 1.0) { // Disc / Sun
            let intensity = smoothstep(radius, radius - softness, angle);
            final_color += body.color * intensity * body.color.w;
            
        } else if (body.direction.w == 2.0) { // Crescent / Moon
            let main_disc = smoothstep(radius, radius - softness, angle);
            
            // Subtracted disc for the crescent shape
            let offset_dir = normalize(dir + vec3<f32>(body.params.z, body.params.z, 0.0));
            let offset_angle = acos(clamp(dot(ray, offset_dir), -1.0, 1.0));
            let mask = smoothstep(radius, radius - softness, offset_angle);
            
            let crescent = clamp(main_disc - mask, 0.0, 1.0);
            final_color += body.color * crescent * body.color.w;
        }
    }

    return vec4<f32>(final_color.rgb, 1.0);
}