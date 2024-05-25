pub mod render_target;
pub mod swirl_material;

use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping::TonyMcMapface;
use bevy::math::Quat;
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, Mesh, OrthographicProjection, Rectangle, RegularPolygon, Res, ResMut, Transform};
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};
use bevy::utils::default;
use bevy_defer::signals::Signals;
use crate::elements2d::tunnelgon::{CancelAnim, Tunnelgon, TunnelgonMaterial, TunnelgonParams};
use crate::hexagon::HexagonDefinition;
use crate::propagating_render_layers::PropagatingRenderLayers;
use crate::swirl::render_target::SwirlRenderTarget;
use crate::swirl::swirl_material::{SwirlMaterial, SwirlParams};

pub struct SwirlPlugin;

impl Plugin for SwirlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SwirlRenderTarget>();
        app.add_plugins(Material2dPlugin::<SwirlMaterial>::default());
        app.add_systems(Startup, setup_swirl);
        app.add_event::<UpdateSwirlParams>();
        app.add_systems(Update, (update_swirl_params_event));
    }
}

#[derive(Component)]
pub struct SwirlRenderer;

fn setup_swirl(
    mut commands: Commands,
    mut materials: ResMut<Assets<SwirlMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    rt: Res<SwirlRenderTarget>,
) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::Fixed { width: 1., height: 1. },
                ..default()
            },
            camera: Camera {
                order: -1000,
                target: RenderTarget::Image(rt.render_target.clone()),
                clear_color: Color::rgba(0., 0., 0., 0.).into(),
                hdr: true,
                ..default()
            },
            //tonemapping: TonyMcMapface,
            ..default()
        },
        //BloomSettings::NATURAL,
        PropagatingRenderLayers { render_layers: RenderLayers::layer(4) }
    ));

    let mesh = Mesh2dHandle(meshes.add(
        Rectangle::new(1., 1.)
    ));


    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(SwirlMaterial {
                prev: rt.render_target.clone(),
                params: SwirlParams::default(),
            }),
            ..default()
        },
        PropagatingRenderLayers { render_layers: RenderLayers::layer(4) },
        SwirlRenderer,
    ));
}

#[derive(Event)]
pub struct UpdateSwirlParams{
    pub new_params: SwirlParams
}

pub fn update_swirl_params_event(
    mut query: Query<&Handle<SwirlMaterial>, With<SwirlRenderer>>,
    mut materials: ResMut<Assets<SwirlMaterial>>,
    mut ev_reader: EventReader<UpdateSwirlParams>,
) {
    for ev in ev_reader.read() {
        if let Ok(mat_handle) = query.get_single_mut() {
            let material = materials.get_mut(mat_handle).unwrap();
            material.params = ev.new_params.clone();
        }
    }
}