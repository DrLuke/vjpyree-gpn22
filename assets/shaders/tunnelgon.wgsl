#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0) var<uniform> foo: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = mesh.uv;
    return vec4<f32>(uv.x, uv.y, sin(globals.time), 1.);
}