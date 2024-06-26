//! Pull all entities to the center

use bevy::math::Vec3;
use bevy::prelude::{Children, Commands, Entity, Event, EventReader, Query, Transform, With};
use bevy::utils::default;
use bevy_egui::egui::debug_text::print;
use bevy_rapier3d::prelude::{ExternalForce, ExternalImpulse, RigidBody};
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::{HexagonPhysicsElement, PhysicsHexagon};

#[derive(Event)]
pub struct CenterPullEvent {
    pub affected_hexagons: Vec<HexagonDefinition>,
    pub strength: f32,
}

impl Default for CenterPullEvent {
    fn default() -> Self {
        Self {
            affected_hexagons: vec![],
            strength: 1600000.,
        }
    }
}

pub fn center_pull_system(
    mut center_pull_event_reader: EventReader<CenterPullEvent>,
    mut commands: Commands,
    children_query: Query<(&PhysicsHexagon, &Children)>,
    physics_element_query: Query<&Transform, (With<HexagonPhysicsElement>, With<RigidBody>)>,
) {
    for event in center_pull_event_reader.read() {
        for (physics_hexagon, children) in children_query.iter() {
            if event.affected_hexagons.contains(&physics_hexagon.hexagon_definition) {
                for child in children {
                    if let Ok(transform) = physics_element_query.get(*child) {
                        let mut direction = transform.translation;
                        direction.z = 0.;
                        commands.entity(*child).insert(
                            ExternalImpulse {
                                impulse: -direction*event.strength,
                                ..default()
                            }
                        );
                    }
                }
            }
        }
    }
}