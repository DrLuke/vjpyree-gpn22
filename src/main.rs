mod hexagon;
mod eyes;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::eyes::EyesPlugin;
use crate::hexagon::HexagonPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        //.add_plugins(HexagonPlugin)
        .add_plugins(EyesPlugin)
        .run();
}