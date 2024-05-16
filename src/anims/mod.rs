mod tubes;
mod meta;

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
use crate::anims::meta::{tunnelgon_laser_cycle_meta_anim, tunnelgon_laser_figure_eight_meta_anim, TunnelgonLaserCycleMetaAnim, TunnelgonLaserFigureEightMetaAnim};
use crate::anims::tubes::{anim1, center_sweep_out};
use crate::beat::osc_beat_receiver_system;
use crate::hexagon::HexagonDefinition;


pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimColors {
            primary: Color::RED,
            secondary: Color::BLUE * 0.3,
        });
        app.add_systems(Startup, main_anim_loop);
        app.add_event::<OneshotAnimEvent>();
        app.add_systems(PreUpdate, react_to_event::<OneshotAnimEvent>);
        app.init_resource::<TunnelgonLaserCycleMetaAnim>();
        app.init_resource::<TunnelgonLaserFigureEightMetaAnim>();
        app.add_systems(Update, (
            tunnelgon_laser_cycle_meta_anim,
            tunnelgon_laser_figure_eight_meta_anim,
        ).after(osc_beat_receiver_system));
    }
}

#[derive(Resource, Copy, Clone, Default)]
pub struct AnimColors {
    primary: Color,
    secondary: Color,
}

/* Anim system:
Types of animations
  * One-shots triggering on beat
  * Continuous animations (only 1 at a time)
 */

#[derive(Copy, Clone)]
pub enum OneshotAnim {
    // Tubes
    CenterOutSweep(f32),
    CenterInSweep(f32),
}

#[derive(Event, Clone)]
pub struct OneshotAnimEvent {
    pub anims: Vec<OneshotAnim>,
    pub affected_hexagons: Vec<HexagonDefinition>,
}

fn main_anim_loop(
    mut commands: Commands
) {
    commands.spawn_task(|| async move {
        oneshot_anim_loop().await;
        Ok(())
    });
    commands.spawn_task(|| async move {
        let mut oneshots: Vec<Pin<Box<dyn Future<Output=_>>>> = vec![Box::pin(anim1(1.))];

        oneshots.pop().unwrap().await;
        Ok(())
    });
}

async fn oneshot_anim_loop() {
    let mut events = world().event_stream::<OneshotAnimEvent>();
    while let Some(ev) = events.next().await {
        let current_time = world().resource::<Time<Real>>().get(|t| t.elapsed_seconds()).await.unwrap();
        for anim in ev.anims {
            let fut = match anim {
                OneshotAnim::CenterOutSweep(speed) => { spawn(center_sweep_out(speed, current_time)) }
                OneshotAnim::CenterInSweep(speed) => { spawn(center_sweep_out(speed, current_time)) }
            };
        }
    }
    error!("Oneshot event reader finished!");
}