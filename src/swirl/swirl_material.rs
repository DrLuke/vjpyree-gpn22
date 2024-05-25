use bevy::asset::{Asset, Handle};
use bevy::prelude::{Color, Image, TypePath};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::sprite::Material2d;
use crate::elements2d::tunnelgon::TunnelgonMaterial;

#[derive(Clone, Debug, ShaderType)]
pub struct SwirlParams {
    pub offset_strength: f32,
    pub fb_rot: f32,
    pub uv_scale: f32,
    pub col_rot: Color,
    pub hex: f32,
    pub circle: f32,
    pub cross: f32,
    pub cross_radius: f32,
    pub thiccness: f32,
    pub fb_strength: f32,
    pub palette: f32,
}

impl Default for SwirlParams {
    fn default() -> Self {
        Self {
            offset_strength: 0.5,
            fb_rot: 0.5,
            uv_scale: 1.,
            col_rot: Color::WHITE,
            hex: 0.2,
            circle: 0.3,
            cross: 0.4,
            cross_radius: 0.5,
            thiccness: 0.05,
            fb_strength: 0.3,
            palette: 1.,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SwirlMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub prev: Handle<Image>,
    #[storage(2, read_only)]
    pub params: SwirlParams,
}

impl Material2d for SwirlMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/swirl.wgsl".into()
    }
}

