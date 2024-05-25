#import bevy_sprite::{
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}

@group(2) @binding(0) var prev: texture_2d<f32>;
@group(2) @binding(1) var prev_sampler: sampler;
@group(2) @binding(2) var<storage, read> params: SwirlParams;


struct SwirlParams {
    offset_strength: f32,
    fb_rot: f32,
    uv_scale: f32,
    col_rot: vec4<f32>,
    hex: f32,
    circle: f32,
    cross: f32,
    cross_radius: f32,
    thiccness: f32,
    fb_strength: f32,
    palette: f32,
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

fn rot3(axis: vec3<f32>, angle: f32) -> mat3x3<f32> {
    let an = normalize(axis);
    let s = sin(angle);
    let c = cos(angle);
    let oc = 1.0 - c;

    return mat3x3<f32>(oc * axis.x * axis.x + c, oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s,
    oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z - axis.x * s,
    oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c);
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

fn rgb2hsv(c: vec3<f32>) -> vec3<f32> {
    let K = vec4<f32>(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    let p = mix(vec4<f32>(c.bg, K.wz), vec4<f32>(c.gb, K.xy), step(c.b, c.g));
    let q = mix(vec4<f32>(p.xyw, c.r), vec4<f32>(c.r, p.yzx), step(p.x, c.r));

    let d = q.x - min(q.w, q.y);
    let e = 1.0e-10;
    return vec3<f32>(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

fn uvcscale(uv: vec2<f32>, scale: f32) -> vec2<f32> {
   return (uv - vec2<f32>(0.5)) * scale + vec2<f32>(0.5);
}

fn uvcrot(uv: vec2<f32>, angle: f32) -> vec2<f32> {
    return (uv - vec2<f32>(0.5)) * rot2(angle) + vec2<f32>(0.5);
}

fn uvcarot(uv: vec2<f32>, angle: f32) -> vec2<f32> {
    let res = vec2<f32>(1920., 610.);
    let aspect = res.x/res.y;
    return (((uv - vec2<f32>(0.5)) * vec2<f32>(aspect, 1.) ) * rot2(angle) + vec2<f32>(0.5)) * vec2<f32>(1./aspect, 1.);
}

fn pal(palette: f32, t: f32) -> vec3<f32> {
    if (palette <= 0.) {
        return palette1(t);
    }
    if (palette <= 1.) {
        return palette2(t);
    }
    if (palette <= 2.) {
        return palette3(t);
    }
    if (palette <= 3.) {
        return palette4(t);
    }
    if (palette <= 4.) {
        return palette5(t);
    }
    if (palette <= 5.) {
        return palette6(t);
    }
    return palette1(t);
}

fn sdHexagon( a: vec2<f32>, r: f32 ) -> f32
{
    let k = vec3<f32>(-0.866025404,0.5,0.577350269);
    var p = abs(a);
    p -= 2.0*min(dot(k.xy,p),0.0)*k.xy;
    p -= vec2(clamp(p.x, -k.z*r, k.z*r), r);
    return length(p)*sign(p.y);
}

fn sdCircle(p: vec2<f32>, r:f32) -> f32
{
    return length(p) - r;
}

fn sdRoundedX( a: vec2<f32>, w: f32, r: f32 ) -> f32
{
    let p = abs(a);
    return length(p-min(p.x+p.y,w)*0.5) - r;
}

const CONTOUR = 0.01;
fn edge(d: f32, thiccness: f32) -> f32
{
    return smoothstep(-thiccness - CONTOUR, -thiccness, d) * (1-smoothstep(thiccness, thiccness + CONTOUR, d));
}

fn mask_func(uvc: vec2<f32>, hex: f32, circle: f32, roundedx: f32, roundedx_radius: f32, thiccness: f32) -> f32
{
    var o = 0.;
    o += edge(sdHexagon(uvc*rot2(PI/6.), hex/3.), thiccness);
    o += edge(sdCircle(uvc, circle*0.4), thiccness)*2.;
    o += edge(sdRoundedX(uvc*rot2(PI/12.), roundedx*0.5, roundedx_radius*0.2), thiccness)*3.;
    return o;
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = mesh.uv;
    let uvc: vec2<f32> = uv - vec2<f32>(0.5);

    // Prev
    let prev_sample = textureSample(prev, prev_sampler, uv); // 1:1 sample
    let prev_hsv = rgb2hsv(prev_sample.rgb);

    // Feedback sampler effects
    var hsv_angle = prev_hsv.x * 3.14159 * 4. + atan2(uvc.y, uvc.x)*1.;
    var sample_offset = vec2<f32>(cos(hsv_angle), sin(hsv_angle)) * 0.001 * params.offset_strength;
    var fb_uv = uvcscale(uvcrot(uv, params.fb_rot * 0.01 / length(uvc)), params.uv_scale) - sample_offset;
    var fb_sample = textureSample(prev, prev_sampler, fb_uv);

    // SDF mask
    let mask = mask_func(uvc, params.hex, params.circle, params.cross, params.cross_radius, params.thiccness);

    let col = pal(params.palette, mask*0.4 + length(uvc)*10.);
    //let col = vec3<f32>(1.);
    fb_sample = vec4<f32>(fb_sample.rgb*rot3(params.col_rot.xyz, params.col_rot.w), fb_sample.a);
    fb_sample = clamp(fb_sample, vec4<f32>(0.), vec4<f32>(1.4));

    return vec4<f32>(col*step(0.3, mask), 1.) + fb_sample*params.fb_strength * (1.-step(0.3, mask));
}