#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0)
var t: texture_2d<f32>;
@group(0) @binding(1)
var ts: sampler;

struct PixelateSettings {
    block_size: f32,
    _padding1: f32,
    _padding2: f32,
    _padding3: f32,
};
@group(0) @binding(2)
var<uniform> pixelate: PixelateSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(textureDimensions(t));

    let width_height_over_block_size = resolution / max(1.0, pixelate.block_size);

    var uv = in.uv + 0.5;
    uv *= width_height_over_block_size;
    uv = floor(uv);
    uv /= width_height_over_block_size;
    uv -= 0.5;

    return textureSample(t, ts, uv); 
}