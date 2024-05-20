#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0) var prev: texture_2d<f32>;
@group(2) @binding(1) var prev_sampler: sampler;
@group(2) @binding(2) var<storage, read> params: TunnelgonParams;


struct TunnelgonParams {
    rings_pos: array<f32, 8>,
    rings_amp: array<f32, 8>,
    laser: array<f32, 8>,
    spiral_freq: f32,
    spiral_skew: f32,
    spiral_dir: f32,
}

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

fn laser(uvt: vec2<f32>, index: f32, intensity: f32) -> f32 {
    return ((1.-smoothstep(0.005, 0.02, abs((uvt.x - (index+3.5)/3. - 0.5)%2. + 0.5))))*intensity;
}
fn ring(uvt: vec2<f32>, pos: f32, intensity: f32) -> f32 {
    return (1-smoothstep(0.005, 0.01, abs(uvt.y-(pos*1.5 + 0.1))))*intensity;
}

fn rot2(a: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(a), -sin(a), sin(a), cos(a));
}

// Color palette by Inigo Quilezles https://iquilezles.org/articles/palettes/
fn palette(t: f32, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    return a + b*cos( 6.28318*(c*t+d) );
}
fn palette1(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(1.), vec3<f32>(0.00, 0.33, 0.67));
}
fn palette2(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.8, 0.5, 0.4), vec3<f32>(0.2, 0.4, 0.2), vec3<f32>(2.0, 1.0, 1.0), vec3<f32>(0.00, 0.25, 0.25));
}
fn palette3(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(2.0, 1.0, 0.0), vec3<f32>(0.50, 0.20, 0.25));
}
fn palette4(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(1.), vec3<f32>(0.00, 0.10, 0.20));
}
fn palette5(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(1.0, 1.0, 0.5), vec3<f32>(0.80, 0.90, 0.30	));
}
fn palette6(t: f32) -> vec3<f32> {
    // FREESTYLE
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(2.0, 1.0, 0.0), vec3<f32>(0.50, 0.20, 0.25));
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = mesh.uv;

    let samp = textureSample(prev, prev_sampler, mesh.position.xy/vec2<f32>(1920,1080));

    let uvc: vec2<f32> = uv - vec2<f32>(0.5);

    let uvt = tunnel(uvc, 0.1);

    var mask = 1.-smoothstep(0.15, 0.25, abs((uvt.y*12. + globals.time + uvt.x*-3.) % 1. - 0.5));
    let fog = 1.-smoothstep(0., 1.5, uvt.y);
    let fog_laser = 1.-smoothstep(0.5, 2., uvt.y);

    // Laser
    let lasermask = laser(uvt, 0., params.laser[0]) + laser(uvt, 1., params.laser[1])
    + laser(uvt, 2., params.laser[2]) + laser(uvt, 3., params.laser[3])
    + laser(uvt, 4., params.laser[4]) + laser(uvt, 5., params.laser[5]);

    let ringmask = ring(uvt, params.rings_pos[0], params.rings_amp[0]) +
    ring(uvt, params.rings_pos[1], params.rings_amp[1]) +
    ring(uvt, params.rings_pos[2], params.rings_amp[2]) +
    ring(uvt, params.rings_pos[3], params.rings_amp[3]) +
    ring(uvt, params.rings_pos[4], params.rings_amp[4]) +
    ring(uvt, params.rings_pos[5], params.rings_amp[5]) +
    ring(uvt, params.rings_pos[6], params.rings_amp[6]) +
    ring(uvt, params.rings_pos[7], params.rings_amp[7]);

    var off_samp = textureSample(prev, prev_sampler, mesh.position.xy/vec2<f32>(1920,1080) + samp.xy*0.001*rot2(length(uvc*10.)));

    var out = vec4<f32>(abs(palette4(uvt.y+globals.time*0.2)*(mask-lasermask)*fog*2. + palette6(uvt.y*0.5-globals.time*4.)*lasermask*fog_laser*150. + palette1(uvt.y*0.5)*ringmask*fog*150.), 1);
    //out = mix(out + off_samp * 0.8, out, 1.-smoothstep(0.1, 0.2, length(out.rgb)));
    return out;
    //return vec4<f32>(params.rings_pos[0], params.rings_amp[0]*0, 0, 1);
    //return vec4<f32>(lasermask, mask*fog*0.8745098039215686, mask*fog*0.1843137254901961, 1.)*0.9 ;
}