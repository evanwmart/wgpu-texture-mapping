// shader.wgsl

@group(0) @binding(0)
var<uniform> transform: mat4x4<f32>; // Transformation matrix

@group(0) @binding(1)
var texture_binding: texture_2d<f32>;

@group(0) @binding(2)
var texture_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>, // UV coordinates for texture sampling
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 4>(
        vec2<f32>(-0.5, -0.5), // Bottom-left
        vec2<f32>(0.5, -0.5),  // Bottom-right
        vec2<f32>(-0.5, 0.5),  // Top-left
        vec2<f32>(0.5, 0.5)    // Top-right
    );

    var uvs = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 1.0), // Bottom-left
        vec2<f32>(1.0, 1.0), // Bottom-right
        vec2<f32>(0.0, 0.0), // Top-left
        vec2<f32>(1.0, 0.0)  // Top-right
    );

    var indices = array<u32, 6>(
        0, 1, 2, // First triangle
        2, 1, 3  // Second triangle
    );

    let index = indices[in_vertex_index];
    let pos = vec4<f32>(positions[index], 0.0, 1.0);

    var output: VertexOutput;
    output.clip_position = transform * pos;
    output.uv = uvs[index];
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture using the UV coordinates
    let tex_color = textureSample(texture_binding, texture_sampler, in.uv);
    return tex_color;
}
