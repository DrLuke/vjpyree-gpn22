//! Implement gravity to be local to each hexagon

use bevy::hierarchy::Children;
use bevy::prelude::{Changed, Commands, Component, Entity, Query, Transform, Vec3, With};
use bevy::utils::default;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::prelude::ExternalForce;
use crate::physics_hexagon::{HexagonPhysicsElement, PhysicsHexagon};

#[derive(Component)]
pub struct LocalGravity {
    pub gravity: Vec3,
}

pub fn local_gravity_system(
    mut commands: Commands,
    physics_hexagon_query: Query<(&Transform, &LocalGravity, &Children), (With<PhysicsHexagon>, Changed<Transform>, Changed<LocalGravity>)>,
    physics_element_query: Query<Entity, (With<HexagonPhysicsElement>, With<RigidBody>)>,
) {
    for (transform, local_gravity, children) in physics_hexagon_query.iter() {
        let applied_gravity_force = transform.rotation.mul_vec3(local_gravity.gravity);
        for child in children {
            if let Ok(entity) = physics_element_query.get(*child) {
                commands.entity(entity).insert(
                    ExternalForce {
                        force: applied_gravity_force,
                        ..default()
                    }
                );
            }
        }
    }
}