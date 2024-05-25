#![feature(future_join)]

mod hexagon;
mod physics_hexagon;
mod propagating_render_layers;
mod gui;
pub mod elements2d;
pub mod parameter_animation;
pub mod traktor_beat;
pub mod beat;
pub mod anims;
mod render_main;
pub mod swirl;

use bevy::app::MainScheduleOrder;
use bevy::core::Zeroable;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_defer::AsyncPlugin;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_rosc::BevyRoscPlugin;
use crate::anims::AnimPlugin;
use crate::parameter_animation::ParameterAnimationPlugin;
use crate::beat::OscBeatReceiverPlugin;
use crate::elements2d::Elements2DPlugin;
use crate::gui::GuiPlugin;
use crate::hexagon::HexagonPlugin;
use crate::physics_hexagon::PhysicsHexagonPlugin;
use crate::propagating_render_layers::{PropagatingRenderLayersPlugin};
use crate::render_main::RenderMainPlugin;
use crate::swirl::SwirlPlugin;
use crate::traktor_beat::TraktorPlugin;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clear;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuiUpdate;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaAnimUpdate;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncUpdate1;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncUpdate2;

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AsyncUpdate3;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(AsyncPlugin::default_settings()
            .run_in::<()>(AsyncUpdate1)
            .run_in::<()>(AsyncUpdate2)
            .run_in::<()>(AsyncUpdate3)
        )
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::Z * -9.81 * 100.,

            ..RapierConfiguration::new(10.)
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PropagatingRenderLayersPlugin)
        .add_plugins(HexagonPlugin)
        .add_plugins(RenderMainPlugin)
        .add_plugins(PhysicsHexagonPlugin)
        .add_plugins(Elements2DPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(ParameterAnimationPlugin)
        .add_plugins((
            BevyRoscPlugin::new("0.0.0.0:31337").unwrap(),
            TraktorPlugin,
            OscBeatReceiverPlugin::default(),
        ))
        .add_systems(Startup, startup)
        .add_plugins(AnimPlugin)
        .add_plugins(SwirlPlugin)
    ;


    app.init_schedule(Clear);
    app.world.resource_mut::<MainScheduleOrder>()
        .insert_after(First, Clear);

    app.init_schedule(GuiUpdate);
    app.world.resource_mut::<MainScheduleOrder>()
        .insert_after(PreUpdate, GuiUpdate);

    app.init_schedule(MetaAnimUpdate);
    app.world.resource_mut::<MainScheduleOrder>()
        .insert_after(GuiUpdate, MetaAnimUpdate);

    app.init_schedule(AsyncUpdate1);
    app.world.resource_mut::<MainScheduleOrder>()
        .insert_after(Update, AsyncUpdate1);

    app.init_schedule(AsyncUpdate2);
    app.world.resource_mut::<MainScheduleOrder>()
        .insert_after(AsyncUpdate1, AsyncUpdate2);

    app.init_schedule(AsyncUpdate3);
    app.world.resource_mut::<MainScheduleOrder>()
        .insert_after(AsyncUpdate2, AsyncUpdate3);

    app.run();
}

fn startup(
    mut ambient_light: ResMut<AmbientLight>
) {
    ambient_light.brightness = 0.0;
}