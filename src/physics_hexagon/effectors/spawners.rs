use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy_rapier3d::dynamics::{Ccd, RigidBody};
use bevy_rapier3d::geometry::Collider;
use rand::thread_rng;
use crate::physics_hexagon::effectors::PhysHexSettings;
use crate::physics_hexagon::{HexagonPhysicsElement, PhysicsHexagon};
use crate::propagating_render_layers::PropagatingRenderLayers;

pub fn spawners_eyes(
    mut commands: Commands,
    phys_hex_query: Query<Entity, With<PhysicsHexagon>>,
    mut physics_element_query: Query<Entity, (With<HexagonPhysicsElement>, With<RigidBody>)>,
    settings: Res<PhysHexSettings>,
    asset_server: Res<AssetServer>,
) {
    let phyics_hexagon_entity = phys_hex_query.get_single().unwrap();

    let mut counter = 0;
    for entity in physics_element_query.iter() {
        counter += 1;
        if counter > settings.eye_count {
            commands.entity(entity).despawn_recursive();
        }
    }


    if counter < settings.eye_count {
        let mut rng = thread_rng();

        let eye_01: Handle<Mesh> = asset_server.load("eye_01.glb#Mesh0/Primitive0");
        let eye_01_material: Handle<StandardMaterial> = asset_server.load("eye_01.glb#Material0");

        while counter < settings.eye_count {
            let entity = commands.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(
                        0.,
                        0.,
                        100.),
                    ..default()
                },
                HexagonPhysicsElement,
                RigidBody::Dynamic,
                Ccd::enabled(),
                Collider::ball(50.),
                PropagatingRenderLayers { render_layers: RenderLayers::layer(1) },
            )).id();

            let eye_mesh = commands.spawn((
                PbrBundle {
                    mesh: eye_01.clone(),
                    material: eye_01_material.clone(),
                    transform: Transform::from_scale(Vec3::new(4000., 4000., 4000.)),
                    ..default()
                }
            )).id();
            commands.entity(phyics_hexagon_entity).push_children(&[entity]);
            commands.entity(entity).push_children(&[
                eye_mesh
            ]);

            counter += 1;
        }
    }
}