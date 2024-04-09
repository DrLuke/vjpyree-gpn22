mod hexagon_definition;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
pub use hexagon_definition::HexagonDefinition;

pub struct HexagonPlugin;

impl Plugin for HexagonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hexagons);
    }
}

fn spawn_hexagons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scaling_mode: ScalingMode::Fixed { width: 1920., height: 1080. },
            ..Default::default()
        },
        ..Camera2dBundle::default()
    });

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

        commands.spawn(MaterialMesh2dBundle {
            mesh,
            material: materials.add(Color::hsl(0., 0.95, 0.7)),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT to +X_EXTENT.
                HexagonDefinition::center(&hexagon).x - 1920./2.,
                HexagonDefinition::center(&hexagon).y - 1080./2.,
                0.0,
            ).with_rotation(Quat::from_rotation_z(PI/6.)),
            ..default()
        });
    }
}