use std::f32::consts::PI;
use std::future::join;
use bevy::asset::Assets;
use bevy::hierarchy::Children;
use bevy::input::ButtonState;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{Color, Commands, Entity, EventReader, GlobalTransform, KeyCode, Local, Parent, Query, Real, Res, ResMut, Resource, Time, With};
use bevy_defer::{AsyncAccess, AsyncCommandsExtension, AsyncFailure, AsyncResult, signal_ids, world};
use bevy_defer::access::AsyncEntityMut;
use bevy_defer::reactors::Reactors;
use bevy_defer::sync::oneshot::ChannelOut;
use bevy_egui::systems::InputEvents;
use futures::future::join_all;
use noise::{NoiseFn, OpenSimplex, Perlin};
use rand::{Rng, thread_rng};
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
    beat_accum: f32,
    beat_accum_pt1: f32,
    pub punch: bool,
    punch_counter: usize,
    pub sweep_out: bool,
    pub sweep_in: bool,
    sweep_accum: f32,
    pub punch2: bool,
    punch2_counter: usize,
    pub punch3: bool,
    punch3_counter: usize,
    pub punch4: bool,
    punch4_counter: usize,
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

pub fn tube_punch_2(
    mut query: Query<(&LedTube, &Children)>,
    mut params: ResMut<TubesWaveAnims>,
    mut beat_reader: EventReader<BeatEvent>,
    mut commands: Commands,
    colors: Res<AnimColors>,
    mut reactors: ResMut<Reactors>,
) {
    if !params.punch2 { return; }
    for _ in beat_reader.read() {
        let signal = reactors.get_named::<CancelPunch>("cancel_punch");
        signal.send(true);

        let anim_indices = vec![
            vec![Six, Fourteen, Thirteen],
            vec![Eight, Sixteen, Eleven],
            vec![Ten, Seventeen, Nine],
            vec![Twelve, Fifteen ,Seven],
            //vec![5,6,7,16,17,18],
            //vec![3,4,8,9,14,15,19,20],
            //vec![1,2,10,11,12,13,21,22],
        ];

        let relevant_indices = &anim_indices[params.punch2_counter];
        params.punch2_counter = (params.punch2_counter + 1) % 4;

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

pub fn tube_punch_3(
    mut query: Query<(&LedTube, &Children)>,
    mut params: ResMut<TubesWaveAnims>,
    mut beat_reader: EventReader<BeatEvent>,
    mut commands: Commands,
    colors: Res<AnimColors>,
    mut reactors: ResMut<Reactors>,
) {
    if !params.punch3 { return; }
    for _ in beat_reader.read() {
        let signal = reactors.get_named::<CancelPunch>("cancel_punch");
        signal.send(true);

        let anim_indices = vec![
            vec![Six, Eight, Fourteen, Sixteen, Eleven, Thirteen],
            vec![Ten, Twelve, Seven, Nine, Fifteen, Seventeen],
            vec![Five, Three, Four, Eighteen, Nineteen, Twenty],
            vec![One, Two, Twentyone, Twentytwo],
            //vec![5,6,7,16,17,18],
            //vec![3,4,8,9,14,15,19,20],
            //vec![1,2,10,11,12,13,21,22],
        ];

        let relevant_indices = &anim_indices[params.punch3_counter];
        params.punch3_counter = (params.punch3_counter + 1) % 4;

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

pub fn tube_punch_4(
    mut query: Query<(&LedTube, &Children)>,
    mut params: ResMut<TubesWaveAnims>,
    mut beat_reader: EventReader<BeatEvent>,
    mut commands: Commands,
    colors: Res<AnimColors>,
    mut reactors: ResMut<Reactors>,
) {
    if !params.punch4 { return; }
    for _ in beat_reader.read() {
        let signal = reactors.get_named::<CancelPunch>("cancel_punch");
        signal.send(true);

        let anim_indices = vec![
            vec![Three, Five, Six, Sixteen, Eighteen, Nineteen],
            vec![Four, Five, Seven, Seventeen, Eighteen, Twenty],
            vec![Eight, Nine, One, Two, Fourteen, Fifteen, Twentyone, Twentytwo],
            vec![Ten, Twelve, Eleven, Thirteen],
            //vec![5,6,7,16,17,18],
            //vec![3,4,8,9,14,15,19,20],
            //vec![1,2,10,11,12,13,21,22],
        ];

        let relevant_indices = &anim_indices[params.punch4_counter];
        params.punch4_counter = (params.punch4_counter + 1) % 4;

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


fn pt1_param(u: f32, y: f32, pt1: f32, dt: f32) -> f32
{
    u + (y - u) * (dt/(pt1+dt))
}
pub fn wave_noise1(
    mut query: Query<(&mut LedTubeLed, &GlobalTransform)>,
    time: Res<Time>,
    mut params: ResMut<TubesWaveAnims>,
    colors: Res<AnimColors>,
    mut beat_reader: EventReader<BeatEvent>,
) {
    if params.wave != 5 { return; }

    let perlin = Perlin::new(1);

    for _ in beat_reader.read() {
        params.beat_accum += 1.;
    }

    params.beat_accum_pt1 = pt1_param(params.beat_accum_pt1, params.beat_accum, 0.03, time.delta_seconds());

    for (mut ltl, gt) in query.iter_mut() {
        let val = perlin.get([gt.translation().x as f64 * 0.01, gt.translation().y as f64  * 0.01, params.beat_accum_pt1 as f64]) as f32;
        ltl.color = colors.primary * val * 2. + colors.secondary * (1.-val) * 0.2;
    }
}

pub fn wave_noise2(
    mut query: Query<(&mut LedTubeLed, &GlobalTransform)>,
    time: Res<Time>,
    mut params: ResMut<TubesWaveAnims>,
    colors: Res<AnimColors>,
    mut beat_reader: EventReader<BeatEvent>,
) {
    if params.wave != 6 { return; }

    let perlin = Perlin::new(1);

    for (mut ltl, gt) in query.iter_mut() {
        let val = perlin.get([gt.translation().x as f64 * 0.01, gt.translation().y as f64  * 0.01, time.elapsed_seconds_f64()]) as f32;
        ltl.color = colors.primary * val * 2. + colors.secondary * (1.-val) * 0.2;
    }
}



pub fn strobe1(
    mut query: Query<(&LedTube, &Children)>,
    mut commands: Commands,
    colors: Res<AnimColors>,
    mut reactors: ResMut<Reactors>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for ev in keyboard_input_events.read() {
        if ev.key_code != KeyCode::Quote || ev.state != ButtonState::Pressed {
            continue;
        }
        let signal = reactors.get_named::<CancelPunch>("cancel_punch");
        signal.send(true);

        let anim_indices = vec![
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Ten,
            Eleven,
            Twelve,
            Thirteen,
            Fourteen,
            Fifteen,
            Sixteen,
            Seventeen,
            Eighteen,
            Nineteen,
            Twenty,
            Twentyone,
            Twentytwo,
        ];

        let mut rng = thread_rng();

        let relevant_indices = vec![
            anim_indices[rng.gen_range(0..anim_indices.len())],
            anim_indices[rng.gen_range(0..anim_indices.len())],
            anim_indices[rng.gen_range(0..anim_indices.len())],
            anim_indices[rng.gen_range(0..anim_indices.len())],
        ];

        let tube_entities: Vec<Vec<Entity>> = query
            .iter()
            .filter(|(led_tube, children)| {
                relevant_indices.contains(&led_tube.get_tube_index())
            })
            .map(|(led_tube, children)| {
                children.iter().cloned().collect()
            })
            .collect();

        let primary_color = Color::WHITE;
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

pub fn strobe2(
    mut query: Query<(&LedTube, &Children)>,
    mut commands: Commands,
    colors: Res<AnimColors>,
    mut reactors: ResMut<Reactors>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    for ev in keyboard_input_events.read() {
        println!("{:?}", ev.key_code);
        if ev.key_code != KeyCode::BracketLeft || ev.state != ButtonState::Pressed {
            continue;
        }
        let signal = reactors.get_named::<CancelPunch>("cancel_punch");
        signal.send(true);

        let anim_indices = vec![
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Ten,
            Eleven,
            Twelve,
            Thirteen,
            Fourteen,
            Fifteen,
            Sixteen,
            Seventeen,
            Eighteen,
            Nineteen,
            Twenty,
            Twentyone,
            Twentytwo,
        ];

        let relevant_indices = anim_indices;

        let tube_entities: Vec<Vec<Entity>> = query
            .iter()
            .filter(|(led_tube, children)| {
                relevant_indices.contains(&led_tube.get_tube_index())
            })
            .map(|(led_tube, children)| {
                children.iter().cloned().collect()
            })
            .collect();

        let primary_color = Color::WHITE;
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
                        time_constant: 0.1,
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