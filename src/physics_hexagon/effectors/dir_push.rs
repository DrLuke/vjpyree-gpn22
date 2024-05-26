//! Pull all entities to one direction

use bevy::math::Vec3;
use bevy::prelude::{Children, Commands, Entity, Event, EventReader, Query, Transform, With};
use bevy::utils::default;
use bevy_egui::egui::debug_text::print;
use bevy_rapier3d::prelude::{ExternalForce, ExternalImpulse, RigidBody};
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::{HexagonPhysicsElement, PhysicsHexagon};

#[derive(Event, Default, Clone)]
pub struct DirPushEvent {
    pub dir: f32,
}

pub fn dir_push_system(
    mut center_pull_event_reader: EventReader<DirPushEvent>,
    mut commands: Commands,
    children_query: Query<(&PhysicsHexagon, &Children)>,
    physics_element_query: Query<&Transform, (With<HexagonPhysicsElement>, With<RigidBody>)>,
) {
    for event in center_pull_event_reader.read() {
        for (physics_hexagon, children) in children_query.iter() {
            for child in children {
                if let Ok(transform) = physics_element_query.get(*child) {
                    let mut direction = Vec3::new(event.dir.sin(), event.dir.cos(), 0.);
                    commands.entity(*child).insert(
                        ExternalImpulse {
                            impulse: direction*120000000.,
                            ..default()
                        }
                    );
                }
            }
        }
    }
}