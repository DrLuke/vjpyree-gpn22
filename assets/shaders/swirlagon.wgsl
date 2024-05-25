#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0) var tex: texture_2d<f32>;
@group(2) @binding(1) var tex_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var samp = textureSample(tex, tex_sampler, mesh.uv);
    samp.a = 1.0;
    return pow(samp, vec4<f32>(2.))*2.;
}