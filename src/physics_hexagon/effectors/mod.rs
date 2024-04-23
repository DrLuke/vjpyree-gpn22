//! Effects for physics objects spawned into hexagons, like pulling to center or dispersing

use bevy::app::{App, Update};
use bevy::prelude::Plugin;
use crate::physics_hexagon::effectors::center_pull::{center_pull_system, CenterPullEvent};
use crate::physics_hexagon::effectors::center_push::{center_push_system, CenterPushEvent};

pub mod center_pull;
pub mod center_push;

pub struct EffectorsPlugin;

impl Plugin for EffectorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CenterPullEvent>();
        app.add_event::<CenterPushEvent>();
        app.add_systems(Update, (center_pull_system, center_push_system));
    }
}