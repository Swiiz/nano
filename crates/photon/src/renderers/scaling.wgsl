// Vertex shader

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    return out;
}

// Fragment shader

@group(0) @binding(0)
var tex: texture_2d<f32>;
@group(0)@binding(1)
var sam: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var rgb_color: vec3<f32> = textureSample(tex, sam, in.tex_coords).xyz;
    var srgb_color: vec3<f32> = pow(rgb_color.xyz, vec3<f32>(2.2));
    return vec4<f32>(srgb_color, 1.0);
}