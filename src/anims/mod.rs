pub mod tubes;
pub mod meta_tunnelgon;

use std::future::{Future, poll_fn, PollFn};
use std::pin::Pin;
use std::task::{Context, Poll};
use bevy::app::{App, Plugin, PostUpdate, Startup};
use bevy::log::error;
use bevy::prelude::{Color, Commands, Event, EventReader, IntoSystemSetConfigs, PreUpdate, Resource, Time, Update, IntoSystemConfigs};
use bevy::tasks::futures_lite::StreamExt;
use bevy::time::Real;
use bevy_defer::{AsyncAccess, AsyncCommandsExtension, AsyncFailure, in_async_context, spawn, world};
use bevy_defer::systems::react_to_event;
use crate::anims::meta_tunnelgon::{tunnelgon_laser_cycle_meta_anim, tunnelgon_laser_figure_eight_meta_anim, tunnelgon_laser_round_the_clock_meta_anim, tunnelgon_laser_sweep_anim, tunnelgon_ring_train_meta_anim, tunnelgon_rings_btf_meta_anim, tunnelgon_rings_ftb_meta_anim, TunnelgonLaserCycleMetaAnim, TunnelgonLaserFigureEightMetaAnim, TunnelgonLaserRoundTheClockMetaAnim, TunnelgonLaserSweepMetaAnim, TunnelgonRingsBTFMetaAnim, TunnelgonRingsFTBMetaAnim, TunnelgonRingsTrainMetaAnim};
use crate::anims::tubes::{anim1, center_sweep_out, TubesWaveAnims, wave_simple, wave_blocky};
use crate::beat::osc_beat_receiver_system;
use crate::hexagon::HexagonDefinition;
use crate::MetaAnimUpdate;


pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimColors {
            primary: Color::RED,
            secondary: Color::BLUE,
        });
        app.init_resource::<TunnelgonLaserCycleMetaAnim>();
        app.init_resource::<TunnelgonLaserFigureEightMetaAnim>();
        app.init_resource::<TunnelgonLaserRoundTheClockMetaAnim>();
        app.init_resource::<TunnelgonLaserSweepMetaAnim>();
        app.init_resource::<TunnelgonRingsFTBMetaAnim>();
        app.init_resource::<TunnelgonRingsBTFMetaAnim>();
        app.init_resource::<TunnelgonRingsTrainMetaAnim>();
        app.add_systems(MetaAnimUpdate, (
            tunnelgon_laser_cycle_meta_anim,
            tunnelgon_laser_figure_eight_meta_anim,
            tunnelgon_laser_round_the_clock_meta_anim,
            tunnelgon_laser_sweep_anim,
            tunnelgon_rings_ftb_meta_anim,
            tunnelgon_rings_btf_meta_anim,
            tunnelgon_ring_train_meta_anim,
        ));
        app.init_resource::<TubesWaveAnims>();
        app.add_systems(MetaAnimUpdate, (
            wave_simple,
            wave_blocky,
        ));
    }
}

#[derive(Resource, Copy, Clone, Default)]
pub struct AnimColors {
    primary: Color,
    secondary: Color,
}
