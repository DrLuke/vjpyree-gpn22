pub mod zoomagon;
pub mod render;

use bevy::app::App;
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, default, OrthographicProjection, Plugin, Res, Startup, Update};
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use crate::elements2d::render::Elements2dRendertarget;
use crate::elements2d::zoomagon::{spawn_zoomagon_system, SpawnZoomagonEvent, zoomagon_system};
use crate::propagating_render_layers::PropagatingRenderLayers;

pub struct Elements2DPlugin;

impl Plugin for Elements2DPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Elements2dRendertarget>();
        app.add_event::<SpawnZoomagonEvent>();
        app.add_systems(Startup, setup_elements_2d);
        app.add_systems(Update, (spawn_zoomagon_system, zoomagon_system));
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
                ..default()
            },
            ..default()
        },
        PropagatingRenderLayers { render_layers: RenderLayers::layer(3) }
    ));
}