use std::f32::consts::PI;
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{Color, ColorMaterial, Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader, Mesh, Quat, Query, RegularPolygon, Res, ResMut, Time, Transform, With};
use bevy::render::view::RenderLayers;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::utils::default;
use crate::hexagon::HexagonDefinition;
use crate::propagating_render_layers::PropagatingRenderLayers;

#[derive(Event)]
pub struct SpawnZoomagonEvent {
    pub affected_hexagons: Vec<HexagonDefinition>,
    pub color: Color,
}

#[derive(Component)]
pub struct Zoomagon {}

pub fn spawn_zoomagon_system(
    mut event_reader: EventReader<SpawnZoomagonEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event in event_reader.read() {
        let material = materials.add(event.color);
        for hexagon_definition in &event.affected_hexagons {
            let mesh = Mesh2dHandle(meshes.add(
                RegularPolygon::new(HexagonDefinition::size(hexagon_definition).x / 2., 6)
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh,
                    material: material.clone(),
                    transform: Transform::from_xyz(
                        // Distribute shapes from -X_EXTENT to +X_EXTENT.
                        HexagonDefinition::center(hexagon_definition).x - 1920. / 2.,
                        HexagonDefinition::center(hexagon_definition).y - 1080. / 2.,
                        0.0,
                    ).with_rotation(Quat::from_rotation_z(PI / 6.)),
                    ..default()
                },
                Zoomagon {},
                PropagatingRenderLayers {render_layers: RenderLayers::layer(3)}
            ));
        }
    }
}

pub fn zoomagon_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Zoomagon>>,
    time: Res<Time>
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.scale = transform.scale - 2.*time.delta_seconds();
        if transform.scale.x < 0.001 {
            commands.entity(entity).despawn_recursive();
        }
    }
}