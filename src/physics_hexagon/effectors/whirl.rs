//! Pull all entities to one direction

use std::f32::consts::PI;
use bevy::math::{Vec3, Vec3Swizzles};
use bevy::prelude::{Children, Commands, Entity, Event, EventReader, Query, Transform, With};
use bevy::utils::default;
use bevy_egui::egui::debug_text::print;
use bevy_rapier3d::prelude::{ExternalForce, ExternalImpulse, RigidBody};
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::{HexagonPhysicsElement, PhysicsHexagon};

#[derive(Event, Default, Clone)]
pub struct WhirlEvent;

pub fn whirl_system(
    mut center_pull_event_reader: EventReader<WhirlEvent>,
    mut commands: Commands,
    children_query: Query<(&PhysicsHexagon, &Children)>,
    physics_element_query: Query<&Transform, (With<HexagonPhysicsElement>, With<RigidBody>)>,
) {
    for event in center_pull_event_reader.read() {
        for (physics_hexagon, children) in children_query.iter() {
            for child in children {
                if let Ok(transform) = physics_element_query.get(*child) {
                    let angle = transform.translation.y.atan2(transform.translation.x) - PI/2. - 0.6;
                    let length = transform.translation.xy().length();

                    let mut direction = Vec3::new(angle.cos(), angle.sin(), 0.);
                    commands.entity(*child).insert(
                        ExternalImpulse {
                            impulse: direction*20000. * length,
                            ..default()
                        }
                    );
                }
            }
        }
    }
}