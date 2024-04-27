mod hexagon_definition;
pub mod render;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_egui::egui::debug_text::print;
pub use hexagon_definition::HexagonDefinition;
use crate::hexagon::render::HexagonRenderTarget;
use crate::physics_hexagon::lights::led_tube::LedTube;
use crate::propagating_render_layers::PropagatingRenderLayers;

pub struct HexagonPlugin;

impl Plugin for HexagonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_hexagons));
        app.add_systems(Update, (spawn_debug_led_tubes));
        app.init_resource::<HexagonRenderTarget>();
    }
}

fn spawn_hexagons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rt: Res<HexagonRenderTarget>,
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
                order: -100,
                target: RenderTarget::Image(rt.render_target.clone()),
                clear_color: Color::rgba(0., 0., 0., 0.).into(),
                ..default()
            },
            ..default()
        },
        PropagatingRenderLayers { render_layers: RenderLayers::layer(2) }
    ));

    let hexagons = [
        HexagonDefinition::Main,
        HexagonDefinition::A1,
        HexagonDefinition::A2,
        HexagonDefinition::A3,
        HexagonDefinition::B1,
        HexagonDefinition::B2,
        HexagonDefinition::B3,
    ];

    for hexagon in hexagons {
        let mesh = Mesh2dHandle(meshes.add(
            RegularPolygon::new(HexagonDefinition::size(&hexagon).x / 2., 6)
        ));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh,
                material: materials.add(Color::rgba(0.3, 1., 0.3, 0.2)),
                transform: Transform::from_xyz(
                    // Distribute shapes from -X_EXTENT to +X_EXTENT.
                    HexagonDefinition::center(&hexagon).x - 1920. / 2.,
                    HexagonDefinition::center(&hexagon).y - 1080. / 2.,
                    0.0,
                ).with_rotation(Quat::from_rotation_z(PI / 6.)),
                ..default()
            },
            PropagatingRenderLayers { render_layers: RenderLayers::layer(2) }
        ));
    }
}

/// Spawn a debug rectangle whenever a LED tube is spawned
fn spawn_debug_led_tubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Transform, &LedTube), Added<LedTube>>,
) {
    let mesh = Mesh2dHandle(meshes.add(
        Rectangle { half_size: Vec2{ x: 173. / 2., y: 4. } }
    ));
    
    for (transform, led_tube) in query.iter() {

        let mut translation = transform.translation.clone();
        translation.x = translation.x - 1920. / 2.;
        translation.y = translation.y - 1080. / 2.;

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: mesh.clone(),
                material: materials.add(Color::rgba(1., 0., 0.1, 1.)),
                transform: transform.clone().with_translation(translation),
                ..default()
            },
            PropagatingRenderLayers { render_layers: RenderLayers::layer(2) }
        ));
    }
}