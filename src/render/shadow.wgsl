@group(0) @binding(0) var<uniform> light_view_proj: mat4x4<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>, 
    @location(3) tangent: vec4<f32>,
    @location(4) color: vec3<f32>,      
};

@vertex
fn vs_main(model: VertexInput) -> @builtin(position) vec4<f32> {
    // Project the 3D position from the Light's perspective!
    return light_view_proj * vec4<f32>(model.position, 1.0);
}
// Note: No fragment shader needed! We only want the depth output.