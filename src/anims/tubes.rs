use std::f32::consts::PI;
use std::future::join;
use bevy::asset::Assets;
use bevy::hierarchy::Children;
use bevy::prelude::{Color, Commands, Entity, EventReader, GlobalTransform, Local, Parent, Query, Real, Res, ResMut, Resource, Time, With};
use bevy_defer::{AsyncAccess, AsyncCommandsExtension, AsyncFailure, AsyncResult, signal_ids, world};
use bevy_defer::access::AsyncEntityMut;
use bevy_defer::reactors::Reactors;
use bevy_defer::sync::oneshot::ChannelOut;
use futures::future::join_all;
use crate::anims::AnimColors;
use crate::beat::BeatEvent;
use crate::elements2d::tunnelgon::{CancelAnim, TunnelgonMaterial};
use crate::parameter_animation::{LinearAnim, ParameterAnimation, Pt1Anim};
use crate::physics_hexagon::lights::led_tube::{LedTube, LedTubeLed, TubeIndex};
use crate::physics_hexagon::lights::led_tube::TubeIndex::{Eight, Eighteen, Eleven, Fifteen, Five, Four, Fourteen, Nine, Nineteen, One, Seven, Seventeen, Six, Sixteen, Ten, Thirteen, Three, Twelve, Twenty, Twentyone, Twentytwo, Two};

#[derive(Resource, Default)]
pub struct TubesWaveAnims {
    pub(crate) wave: usize,
    accum: f32,
    pub punch: bool,
    punch_counter: usize,
    pub sweep_out: bool,
    pub sweep_in: bool,
    sweep_accum: f32,
}

pub fn clear(
    mut query: Query<(&mut LedTubeLed, &GlobalTransform)>,
    colors: Res<AnimColors>,
) {
    for (mut ltl, gt) in query.iter_mut() {
        let x = gt.translation().x;
        ltl.color = colors.secondary.clone() * 0.2;
    }
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
        let x = (gt.translation().x * 2. + gt_ltl.translation().x) / 3.;
        ltl.color = colors.primary * (x * 0.005 - params.accum * 2. * x.signum()).sin().powf(8.)
            + colors.secondary * (0.2 + (x * 0.005 - params.accum * 2. * x.signum() + PI / 2.).sin().powf(8.) * 0.1);
    }
}

signal_ids! {
    pub CancelPunch: bool
}

pub fn tube_punch(
    mut query: Query<(&LedTube, &Children)>,
    mut params: ResMut<TubesWaveAnims>,
    mut beat_reader: EventReader<BeatEvent>,
    mut commands: Commands,
    colors: Res<AnimColors>,
    mut reactors: ResMut<Reactors>,
) {
    if !params.punch { return; }
    for _ in beat_reader.read() {
        let signal = reactors.get_named::<CancelPunch>("cancel_punch");
        signal.send(true);

        let anim_indices = vec![
            vec![Five, Six, Seven, Sixteen, Seventeen, Eighteen],
            vec![Three, Four, Eight, Nine, Fourteen, Fifteen, Nineteen, Twenty],
            vec![One, Two, Ten, Eleven, Twelve, Thirteen, Twentyone, Twentytwo],
            vec![Three, Four, Eight, Nine, Fourteen, Fifteen, Nineteen, Twenty],
            //vec![5,6,7,16,17,18],
            //vec![3,4,8,9,14,15,19,20],
            //vec![1,2,10,11,12,13,21,22],
        ];

        let relevant_indices = &anim_indices[params.punch_counter];
        params.punch_counter = (params.punch_counter + 1) % 4;

        let tube_entities: Vec<Vec<Entity>> = query
            .iter()
            .filter(|(led_tube, children)| {
                relevant_indices.contains(&led_tube.get_tube_index())
            })
            .map(|(led_tube, children)| {
                children.iter().cloned().collect()
            })
            .collect();

        let primary_color = colors.primary;
        let secondary_color = colors.secondary;

        for tube_entity in tube_entities {
            commands.spawn_task(move || async move {
                let signal = world().named_signal::<CancelPunch>("cancel_punch");
                let _ = signal.poll().await; // Discard first message to avoid immediate cancel

                // Spawn PT1 anim
                let pt1_entity = world().spawn_bundle(
                    Pt1Anim {
                        val: 1.3,
                        target: 0.,
                        time_constant: 0.3,
                        //time_constant: 1.,
                    }
                ).await.id();
                let pt1_component = world().entity(pt1_entity).component::<Pt1Anim>();

                let led_tube_entities: Vec<AsyncEntityMut> = tube_entity.iter().map(|child| {
                    world().entity(*child)
                }).collect();

                loop {
                    let (next_val, finished) = match signal.try_read() {
                        Some(true) => (0., true),
                        Some(false) | None => pt1_component.get(|pt1anim| { (pt1anim.get_val(), pt1anim.target_reached()) }).await.unwrap_or((0., true))
                    };

                    let futures: Vec<ChannelOut<AsyncResult<_>>> = led_tube_entities.iter().map(|ent| {
                        ent.component::<LedTubeLed>().set(move |ltl| {
                            let ind = (ltl.get_index() as f32 / 15.) - 0.5;
                            let lum = next_val * (ind * (1.3 - next_val) * 2.).cos();
                            ltl.color = primary_color.clone() * lum + secondary_color.clone() * (1. - lum.min(1.)) * 0.2;
                        })
                    }).collect();
                    let _ = join_all(futures).await;

                    if finished {
                        break;
                    }
                }

                world().entity(pt1_entity).despawn().await;
                Ok(())
            });
        }
    }
}

pub fn sweep(
    mut query: Query<(&mut LedTubeLed, &GlobalTransform)>,
    mut params: ResMut<TubesWaveAnims>,
    mut beat_reader: EventReader<BeatEvent>,
    colors: Res<AnimColors>,
    time: Res<Time<Real>>,
) {
    if !params.sweep_out && !params.sweep_in { return; }

    for _ in beat_reader.read() {
        params.sweep_accum = if params.sweep_out { 0. } else { 1. };
    }

    if params.sweep_out {
        params.sweep_accum += time.delta_seconds() * 2.5;
    } else {
        params.sweep_accum -= time.delta_seconds() * 2.5;
    }

    for (mut ltl, gt) in query.iter_mut() {
        let x = gt.translation().x / (1920. / 2.);
        let x_r = (-x + params.sweep_accum).abs();
        let x_l = (-x - params.sweep_accum).abs();
        let lum_r = if x >= 0. && x > params.sweep_accum { 0. } else { (1. / (x_r * 7. + 0.8)).min(1.3) };
        let lum_l = if x < 0. && x < -params.sweep_accum { 0. } else { (1. / (x_l * 7. + 0.8)).min(1.3) };

        let lum = lum_r + lum_l;

        ltl.color = colors.primary * lum + colors.secondary * (1. - lum) * 0.2;
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

pub async fn center_sweep_out_old(speed: f32, start_time: f32) -> Result<(), AsyncFailure> {
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