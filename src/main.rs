mod hexagon;
mod physics_hexagon;
mod render_out;
mod propagating_render_layers;
mod gui;
mod output_window;

use bevy::core::Zeroable;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use crate::gui::GuiPlugin;
use crate::hexagon::HexagonPlugin;
use crate::output_window::OutputWindowPlugin;
use crate::physics_hexagon::PhysicsHexagonPlugin;
use crate::propagating_render_layers::{PropagatingRenderLayersPlugin};
use crate::render_out::RenderOutPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(OutputWindowPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::Z * -9.81 * 100.,

            ..default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(10.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PropagatingRenderLayersPlugin)
        .add_plugins(HexagonPlugin)
        .add_plugins(RenderOutPlugin)
        .add_plugins(PhysicsHexagonPlugin)
        .add_plugins(GuiPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut ambient_light: ResMut<AmbientLight>
) {
    ambient_light.brightness = 0.0;
}