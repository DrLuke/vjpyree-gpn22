use bevy::prelude::*;
use bevy::render::camera::{ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::hexagon::render::HexagonRenderTarget;
use crate::physics_hexagon::render::PhysicsHexagonRenderTarget;

pub struct RenderOutPlugin;

impl Plugin for RenderOutPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    physics_hexagon_rt: ResMut<PhysicsHexagonRenderTarget>,
    debug_hexagon_rt: ResMut<HexagonRenderTarget>,
) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::Fixed { width: 1., height: 1. },
                ..Default::default()
            },
            ..Camera2dBundle::default()
        },
        RenderLayers::layer(31)
    ));

    // Physics Hexagon
    let material = materials.add(ColorMaterial {
        color: Default::default(),
        texture: Some(physics_hexagon_rt.render_target.clone()),
    });

    let mesh = Mesh2dHandle(meshes.add(
        Rectangle::new(1., 1.)
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material,
            ..default()
        },
        RenderLayers::layer(31)
    ));

    // Debug Hexagons
    let material = materials.add(ColorMaterial {
        color: Default::default(),
        texture: Some(debug_hexagon_rt.render_target.clone()),
    });

    let mesh = Mesh2dHandle(meshes.add(
        Rectangle::new(1., 1.)
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material,
            ..default()
        },
        RenderLayers::layer(31)
    ));
}