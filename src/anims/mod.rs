pub mod tubes;
pub mod meta_tunnelgon;
pub mod meta_phys;
mod bridge;

use std::future::{Future, poll_fn, PollFn};
use std::pin::Pin;
use std::task::{Context, Poll};
use bevy::app::{App, Plugin, PostUpdate, Startup};
use bevy::log::error;
use bevy::prelude::{Color, Commands, Event, EventReader, IntoSystemSetConfigs, PreUpdate, Resource, Time, Update, IntoSystemConfigs, ResMut, Res};
use bevy::tasks::futures_lite::StreamExt;
use bevy::time::Real;
use bevy_defer::{AsyncAccess, AsyncCommandsExtension, AsyncFailure, in_async_context, spawn, world};
use crate::anims::meta_tunnelgon::{tunnelgon_laser_cycle_meta_anim, tunnelgon_laser_figure_eight_meta_anim, tunnelgon_laser_round_the_clock_meta_anim, tunnelgon_laser_sweep_anim, tunnelgon_ring_train_meta_anim, tunnelgon_rings_btf_meta_anim, tunnelgon_rings_ftb_meta_anim, TunnelgonLaserCycleMetaAnim, TunnelgonLaserFigureEightMetaAnim, TunnelgonLaserRoundTheClockMetaAnim, TunnelgonLaserSweepMetaAnim, TunnelgonRingsBTFMetaAnim, TunnelgonRingsFTBMetaAnim, TunnelgonRingsTrainMetaAnim};
use crate::anims::tubes::{TubesWaveAnims, wave_simple, wave_blocky, tube_punch, clear, sweep, tube_punch_2, tube_punch_3, tube_punch_4, wave_noise1, wave_noise2, strobe1, strobe2};
use crate::{Clear, GuiUpdate, MetaAnimUpdate};
use crate::anims::meta_phys::{PhysMetaAnim, push_or_pull_meta_anim, push_pull_meta_anim, sides_meta_anim, up_down, whirl};


pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimColors {
            primary: Color::RED,
            secondary: Color::BLUE,
            anim: 0,
        });
        app.init_resource::<TunnelgonLaserCycleMetaAnim>();
        app.init_resource::<TunnelgonLaserFigureEightMetaAnim>();
        app.init_resource::<TunnelgonLaserRoundTheClockMetaAnim>();
        app.init_resource::<TunnelgonLaserSweepMetaAnim>();
        app.init_resource::<TunnelgonRingsFTBMetaAnim>();
        app.init_resource::<TunnelgonRingsBTFMetaAnim>();
        app.init_resource::<TunnelgonRingsTrainMetaAnim>();
        app.add_systems(Clear, clear);
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
            tube_punch,
            tube_punch_2,
            tube_punch_3,
            tube_punch_4,
            wave_noise1,
            wave_noise2,
            sweep,
            anim_colors,
        ).before(strobe1));
        app.init_resource::<PhysMetaAnim>();
        app.add_systems(MetaAnimUpdate, (
            push_or_pull_meta_anim,
            push_pull_meta_anim,
            sides_meta_anim,
            up_down,
            whirl,
        ));
        app.add_systems(MetaAnimUpdate, (strobe1, strobe2));
    }
}

#[derive(Resource, Copy, Clone, Default)]
pub struct AnimColors {
    pub(crate) primary: Color,
    pub(crate) secondary: Color,
    pub anim: usize,
}

fn anim_colors(
    mut colors: ResMut<AnimColors>,
    time: Res<Time<Real>>,
) {
    if colors.anim == 1 {
        colors.primary = Color::hsl(time.elapsed_seconds() * 100. % 360., 1., 0.5).as_rgba();
        colors.secondary = Color::hsl((time.elapsed_seconds() * 100. + 180.) % 360., 1., 0.5).as_rgba();
    }
}