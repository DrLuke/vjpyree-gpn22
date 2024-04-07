mod hexagon;
mod physics_hexagon;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use crate::hexagon::HexagonPlugin;
use crate::physics_hexagon::PhysicsHexagonPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierConfiguration {
            gravity: Vect::Z * -9.81 * 100.,
            ..default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        //.add_plugins(HexagonPlugin)
        .add_plugins(PhysicsHexagonPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut ambient_light: ResMut<AmbientLight>
) {
    ambient_light.brightness = 0.0;
}