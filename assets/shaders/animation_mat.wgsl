#import bevy_sprite::mesh2d_vertex_output::VertexOutput

// @group(2) @binding(1)
// var texture: texture_2d<f32>;
// @group(2) @binding(2)
// var splr: sampler;
// @group(2) @binding(3)
// var<uniform> index: vec4<f32>;
// // @group(2) @binding(4)
// // var<uniform> length: f32;
// // @group(2) @binding(5)
// // var<uniform> x_offset: f32;
// // @group(2) @binding(6)

// @fragment
// fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
//     let out_rgba = textureSample(texture, splr, in.uv);
//     return vec4<f32>(out_rgba[0] * index[0], out_rgba[1] * index[1], out_rgba[2] * index[2], out_rgba[3]);
// }


@group(2) @binding(1)
var texture: texture_2d<f32>;
@group(2) @binding(2)
var splr: sampler;
@group(2) @binding(3)
var<uniform> ix_length_pad_pad: vec4<f32>;
@group(2) @binding(4)
var<uniform> xoff_yoff_xrep_yrep: vec4<f32>;
@group(2) @binding(5)
var<uniform> rgba: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Unpack stuff
    let ix = ix_length_pad_pad[0];
    let length = ix_length_pad_pad[1];
    let x_offset = xoff_yoff_xrep_yrep[0];
    let y_offset = xoff_yoff_xrep_yrep[1];
    let x_repetitions = xoff_yoff_xrep_yrep[2];
    let y_repetitions = xoff_yoff_xrep_yrep[3];
    let r = rgba[0];
    let g = rgba[1];
    let b = rgba[2];
    let a = rgba[3];

    // Adding 2.0 here because it works, no idea why
    let input_x = (-x_offset + 2.0 + in.uv.x * x_repetitions) % 1.0;
    let input_y = (y_offset + 2.0 + in.uv.y * y_repetitions) % 1.0;
    let index_lower = (1.0 / length) * (ix + 0);
    let index_upper = (1.0 / length) * (ix + 1);
    let out_uv = vec2<f32>(index_lower + (index_upper - index_lower) * input_x, input_y);
    let out_rgba = textureSample(texture, splr, out_uv);

    return vec4<f32>(out_rgba[0] * r, out_rgba[1] * g, out_rgba[2] * b, out_rgba[3] * a);
}
