use std::f32::consts::PI;
use std::future::join;
use bevy::prelude::{Color, EventReader, GlobalTransform, Local, Parent, Query, Real, Res, ResMut, Resource, Time, With};
use bevy_defer::{AsyncAccess, AsyncFailure, world};
use crate::anims::AnimColors;
use crate::beat::BeatEvent;
use crate::parameter_animation::Pt1Anim;
use crate::physics_hexagon::lights::led_tube::{LedTube, LedTubeLed};

#[derive(Resource, Default)]
pub struct TubesWaveAnims {
    pub(crate) wave: usize,
    accum: f32,
}

pub fn wave_simple(
    mut query: Query<(&mut LedTubeLed, &GlobalTransform)>,
    time: Res<Time>,
    mut params: ResMut<TubesWaveAnims>,
    colors: Res<AnimColors>,
    mut beat_reader: EventReader<BeatEvent>,
) {
    if params.wave == 1 { params.accum += time.delta_seconds(); } else if params.wave == 2 { params.accum -= time.delta_seconds(); } else { return; }

    for (mut ltl, gt) in query.iter_mut() {
        let x = gt.translation().x;
        ltl.color = colors.primary * (x * 0.005 - params.accum * 2. * x.signum()).sin().powf(8.)
            + colors.secondary * (0.2 + (x * 0.005 - params.accum * 2. * x.signum() + PI / 2.).sin().powf(8.) * 0.1);
    }
}

pub fn wave_blocky(
    mut query: Query<(&mut LedTubeLed, &GlobalTransform, &Parent)>,
    mut p_query: Query<&GlobalTransform, With<LedTube>>,
    time: Res<Time>,
    colors: Res<AnimColors>,
    mut params: ResMut<TubesWaveAnims>,
) {
    if params.wave == 3 { params.accum += time.delta_seconds(); } else if params.wave == 4 { params.accum -= time.delta_seconds(); } else { return; }
    for (mut ltl, gt_ltl, parent) in query.iter_mut() {
        let gt = p_query.get(parent.get()).unwrap();
        let x = (gt.translation().x*2. + gt_ltl.translation().x) / 3.;
        ltl.color = colors.primary * (x * 0.005 - params.accum * 2. * x.signum()).sin().powf(8.)
            + colors.secondary * (0.2 + (x * 0.005 - params.accum * 2. * x.signum() + PI / 2.).sin().powf(8.) * 0.1);
    }
}

pub async fn anim1(speed: f32) -> Result<(), AsyncFailure> {
    let tubes = world().query::<(&mut LedTubeLed, &GlobalTransform)>();
    let time_resource = world().resource::<Time<Real>>();
    let anim_colors = world().resource::<AnimColors>();

    loop {
        let (time, ac) = join!(
            time_resource.get(|t| t.elapsed_seconds()),
            anim_colors.get(|ac| ac.clone())
        ).await;
        let time: f32 = time.unwrap();
        let ac: AnimColors = ac.unwrap();

        tubes.for_each(move |(mut ltl, gt)| {
            let x = gt.translation().x;
            ltl.color = ac.primary * (x * 0.005 - time * speed * x.signum()).sin().powf(8.) +
                ac.secondary * (x * 0.005 - time * speed * x.signum() + PI / 2.).sin().powf(8.);
        }).await;
    }

    Ok(())
}

pub async fn center_sweep_out(speed: f32, start_time: f32) -> Result<(), AsyncFailure> {
    let tubes = world().query::<(&mut LedTubeLed, &GlobalTransform)>();
    let time_resource = world().resource::<Time<Real>>();
    let anim_colors = world().resource::<AnimColors>();

    loop {
        let (time, ac) = join!(
            time_resource.get(|t| t.elapsed_seconds()),
            anim_colors.get(|ac| ac.clone())
        ).await;
        let time: f32 = time.unwrap() - start_time;
        let ac: AnimColors = ac.unwrap();

        tubes.for_each(move |(mut ltl, gt)| {
            let x = gt.translation().x;
            // Glitter
            //ltl.color = ltl.color + ac.primary * (x * 0.05 - time * speed * x.signum()).sin().powf(16.);
            ltl.color = ltl.color + ac.primary * (x * 0.5 - time * speed * x.signum()).sin().powf(16.) * 5.;
        }).await;
        if time > 0.5 { break; }
    }

    Ok(())
}