#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0) var<uniform> foo: f32;

const PI: f32 = 3.14159;

fn discretize(a: f32, steps: f32) -> f32 {
    return round(a*steps)/steps;
}

fn tunnel(uv: vec2<f32>, size: f32) -> vec2<f32>
{
    let a: f32 = atan2(uv.y, uv.x);
    let h: f32 = ((a + PI/6 + 10*PI) % (PI/3)) - PI/6;
    let r: f32 = sqrt(dot(uv, uv));
    return vec2((a+PI) / PI, ((size * (cos(PI/6)/cos(h))) / r));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = mesh.uv;

    let uvc: vec2<f32> = uv - vec2<f32>(0.5);

    let uvt = tunnel(uvc, 0.1);

    var mask = smoothstep(0.75, 0.75, (uvt.y*8. + globals.time + uvt.x*2.) % 1.);
    let fog = 1.-smoothstep(0., 1.5, uvt.y);

    return vec4<f32>(mask*fog, mask*fog*0.8745098039215686, mask*fog*0.1843137254901961, 1.);
}