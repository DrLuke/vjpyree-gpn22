//! Effects for physics objects spawned into hexagons, like pulling to center or dispersing

use bevy::app::{App, Update};
use bevy::prelude::{Entity, Plugin, Resource};
use crate::physics_hexagon::effectors::center_pull::{center_pull_system, CenterPullEvent};
use crate::physics_hexagon::effectors::center_push::{center_push_system, CenterPushEvent};
use crate::physics_hexagon::effectors::dir_push::{dir_push_system, DirPushEvent};
use crate::physics_hexagon::effectors::eyes_mode::eyes_mode;
use crate::physics_hexagon::effectors::spawners::spawners_eyes;

pub mod center_pull;
pub mod center_push;
pub mod dir_push;
mod eyes_mode;
mod spawners;

pub struct EffectorsPlugin;

impl Plugin for EffectorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CenterPullEvent>();
        app.add_event::<CenterPushEvent>();
        app.add_event::<DirPushEvent>();
        app.add_systems(Update, (center_pull_system, center_push_system, dir_push_system, eyes_mode, spawners_eyes));
        app.insert_resource(PhysHexSettings::default());
    }
}

#[derive(Default, PartialEq, Debug)]
pub enum EyesMode {
    #[default]
    None,
    Stare,
    Crazy
}

#[derive(Default, PartialEq)]
pub enum EffectorsMode {
    #[default]
    None,
    Pull,
    Push,
    Dir(f32),
}

#[derive(Resource, Default)]
pub struct PhysHexSettings {
    pub eye_count: usize,
    pub pills_count: usize,
    pub eyes_mode: EyesMode,
    pub effectors_mode: EffectorsMode,
}