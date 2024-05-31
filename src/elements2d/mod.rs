pub mod zoomagon;
pub mod render;
pub mod tunnelgon;
pub mod swirlagon;
pub mod pedrogon;

use bevy::app::{App, PreUpdate};
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::core_2d::graph::Node2d::Tonemapping;
use bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface;
use bevy::pbr::MaterialPlugin;
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, default, OrthographicProjection, Plugin, Res, Startup, Update};
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::sprite::Material2dPlugin;
use bevy_defer::AsyncExtension;
use crate::elements2d::pedrogon::{SetPedrogonEvent, show_pedrogon, spawn_pedrogon, update_pedrogon};
use crate::elements2d::render::Elements2dRendertarget;
use crate::elements2d::swirlagon::{SetSwirlagonEvent, show_swirlagon_system, spawn_swirlagon, SwirlagonRenderMaterial};
use crate::elements2d::tunnelgon::{CancelAnim, laser_animation_system, LaserAnimationEvent, ring_animation_system, RingAnimationEvent, SetTunnelgonEvent, spawn_tunnelgon_system, tunnelgon_accum, TunnelgonAccum, TunnelgonMaterial};
use crate::elements2d::zoomagon::{spawn_zoomagon_system, SpawnZoomagonEvent, zoomagon_system};
use crate::propagating_render_layers::PropagatingRenderLayers;

pub struct Elements2DPlugin;

impl Plugin for Elements2DPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Elements2dRendertarget>();
        app.add_systems(Startup, (setup_elements_2d, spawn_swirlagon, spawn_pedrogon));
        app.add_event::<SpawnZoomagonEvent>();
        app.add_systems(Update, (spawn_zoomagon_system, zoomagon_system));
        app.add_event::<SetTunnelgonEvent>();
        app.add_event::<LaserAnimationEvent>();
        app.add_event::<RingAnimationEvent>();
        app.add_plugins(Material2dPlugin::<TunnelgonMaterial>::default());
        app.add_systems(Update, (spawn_tunnelgon_system, laser_animation_system, ring_animation_system));
        app.add_event::<SetSwirlagonEvent>();
        app.add_plugins(Material2dPlugin::<SwirlagonRenderMaterial>::default());
        app.add_systems(Update, (show_swirlagon_system));
        app.add_event::<SetPedrogonEvent>();
        app.add_systems(Update, (show_pedrogon, update_pedrogon));
        app.add_systems(Update, tunnelgon_accum);
        app.init_resource::<TunnelgonAccum>();
    }
}

fn setup_elements_2d(
    mut commands: Commands,
    rt: Res<Elements2dRendertarget>,
) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::Fixed { width: 1920., height: 1080. },
                ..default()
            },
            camera: Camera {
                order: -90,
                target: RenderTarget::Image(rt.render_target.clone()),
                clear_color: Color::rgba(0., 0., 0., 0.).into(),
                hdr: true,
                ..default()
            },
            tonemapping: TonyMcMapface,
            ..default()
        },
        BloomSettings::NATURAL,
        PropagatingRenderLayers { render_layers: RenderLayers::layer(3) }
    ));
}