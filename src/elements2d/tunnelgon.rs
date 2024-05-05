use std::f32::consts::PI;
use std::process::Command;
use bevy::asset::Assets;
use bevy::math::Quat;
use bevy::prelude::{Asset, Color, ColorMaterial, Commands, Component, default, DespawnRecursiveExt, Entity, Event, EventReader, Handle, Image, Mesh, Query, RegularPolygon, ResMut, Transform, TypePath, With};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use crate::elements2d::zoomagon::Zoomagon;
use crate::hexagon::HexagonDefinition;
use crate::propagating_render_layers::PropagatingRenderLayers;

#[derive(Component)]
pub struct Tunnelgon {

}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TunnelgonMaterial {
    #[uniform(0)]
    foo: f32
}

impl Material2d for TunnelgonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tunnelgon.wgsl".into()
    }
}

#[derive(Event)]
pub struct SetTunnelgonEvent {
    pub affected_hexagons: Vec<HexagonDefinition>,
}

pub fn spawn_tunnelgon_system(
    mut commands: Commands,
    mut event_reader: EventReader<SetTunnelgonEvent>,
    mut query: Query<Entity, With<Tunnelgon>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TunnelgonMaterial>>,
) {
    for event in event_reader.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for hexagon_definition in &event.affected_hexagons {
            let mesh = Mesh2dHandle(meshes.add(
                RegularPolygon::new(HexagonDefinition::size(hexagon_definition).x / 2., 6)
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh,
                    material: materials.add(TunnelgonMaterial{
                        foo: 1.
                    }),
                    transform: Transform::from_xyz(
                        // Distribute shapes from -X_EXTENT to +X_EXTENT.
                        HexagonDefinition::center(hexagon_definition).x - 1920. / 2.,
                        HexagonDefinition::center(hexagon_definition).y - 1080. / 2.,
                        0.0,
                    ).with_rotation(Quat::from_rotation_z(PI / 6.)),
                    ..default()
                },
                Tunnelgon {},
                PropagatingRenderLayers {render_layers: RenderLayers::layer(3)}
            ));
        }
    }
}