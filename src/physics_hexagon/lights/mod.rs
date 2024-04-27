use bevy::app::App;
use bevy::prelude::{Commands, Plugin, Startup, Update};
use strum::IntoEnumIterator;
use crate::physics_hexagon::lights::led_tube::{spawn_tube, TubeIndex};

pub mod led_tube;

pub struct LightsPlugin;

impl Plugin for LightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_led_tubes);
    }
}

pub fn spawn_led_tubes(
    mut commands: Commands
) {
    for index in TubeIndex::iter() {
        spawn_tube(index, &mut commands)
    }
}