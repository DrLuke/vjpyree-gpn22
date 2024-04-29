use bevy::app::App;
use bevy::hierarchy::Children;
use bevy::prelude::{Commands, Entity, Plugin, Query, Startup, Update, With};
use strum::IntoEnumIterator;
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::lights::led_tube::{LedTube, LedTubeLed, spawn_tube, TubeIndex};
use crate::physics_hexagon::PhysicsHexagon;

pub mod led_tube;
pub mod physical_lights;
pub mod animations;

pub fn spawn_led_tubes(
    mut commands: Commands
) {
    for index in TubeIndex::iter() {
        spawn_tube(index, &mut commands)
    }
}