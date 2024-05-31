use std::collections::HashMap;
use std::f32::consts::PI;
use std::future::Future;
use std::iter::zip;
use std::process::Command;
use std::ptr::read;
use bevy::asset::Assets;
use bevy::log::warn;
use bevy::math::Quat;
use bevy::prelude::{Asset, Color, ColorMaterial, Commands, Component, default, DespawnRecursiveExt, Entity, Event, EventReader, Handle, Image, Mesh, Query, Real, RegularPolygon, Res, ResMut, Resource, Time, Transform, TypePath, With};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_defer::{async_system, AsyncAccess, AsyncCommandsExtension, signal_ids, world};
use bevy_defer::reactors::Reactors;
use bevy_defer::signals::{Receiver, Sender, Signal, Signals, SignalSender};
use crate::beat::BeatEvent;
use crate::parameter_animation::{LinearAnim, ParameterAnimation, Pt1Anim};
use crate::elements2d::render::Elements2dRendertarget;
use crate::elements2d::zoomagon::Zoomagon;
use crate::hexagon::HexagonDefinition;
use crate::propagating_render_layers::PropagatingRenderLayers;

#[derive(Component)]
pub struct Tunnelgon {
    pub params: TunnelgonParams,
    hexagon_definition: HexagonDefinition,
}

#[derive(Clone, Debug, ShaderType)]
pub struct TunnelgonParams {
    pub rings_pos: [f32; 8],
    pub rings_amp: [f32; 8],
    pub laser: [f32; 8],
    pub spiral_freq: f32,
    pub spiral_skew: f32,
    pub spiral_dir: f32,
    pub spiral_accum: f32,
    pub tun_accum: f32,
    pub tun_accum_target: f32,
}

impl Default for TunnelgonParams {
    fn default() -> Self {
        Self {
            rings_pos: [0., 0., 0., 0., 0., 0., 0., 0.],
            rings_amp: [0., 0., 0., 0., 0., 0., 0., 0.],
            laser: [0., 0., 0., 0., 0., 0., 0., 0.],
            spiral_freq: 10.,
            spiral_skew: 6.,
            spiral_dir: 1.,
            spiral_accum: 1.,
            tun_accum: 0.,
            tun_accum_target: 0.,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TunnelgonMaterial {
    #[texture(0)]
    #[sampler(1)]
    prev: Handle<Image>,
    #[storage(2, read_only)]
    params: TunnelgonParams,
}

impl Material2d for TunnelgonMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tunnelgon.wgsl".into()
    }
}

#[derive(Event)]
pub struct SetTunnelgonEvent {
    pub affected_hexagons: Vec<HexagonDefinition>,
}

pub fn spawn_tunnelgon_system(
    mut commands: Commands,
    mut event_reader: EventReader<SetTunnelgonEvent>,
    mut query: Query<Entity, With<Tunnelgon>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TunnelgonMaterial>>,
    rt: Res<Elements2dRendertarget>,
    mut reactors: ResMut<Reactors>,
) {
    for event in event_reader.read() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        for hexagon_definition in &event.affected_hexagons {
            let mesh = Mesh2dHandle(meshes.add(
                RegularPolygon::new(HexagonDefinition::size(hexagon_definition).x / 2., 6)
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh,
                    material: materials.add(TunnelgonMaterial {
                        prev: rt.render_target.clone(),
                        params: TunnelgonParams::default(),
                    }),
                    transform: Transform::from_xyz(
                        // Distribute shapes from -X_EXTENT to +X_EXTENT.
                        HexagonDefinition::center(hexagon_definition).x - 1920. / 2.,
                        HexagonDefinition::center(hexagon_definition).y - 1080. / 2.,
                        0.0,
                    ).with_rotation(Quat::from_rotation_z(PI / 6.)),
                    ..default()
                },
                Tunnelgon {
                    hexagon_definition: hexagon_definition.clone(),
                    params: TunnelgonParams::default(),
                },
                Signals::new()
                    .with_sender::<CancelAnim>(reactors.get_named::<CancelAnim>("cancel_tunnelgon_laser_anim")),
                PropagatingRenderLayers { render_layers: RenderLayers::layer(3) }
            ));
        }
    }
}

// Animations (base)
// Laser
//   Pulse (full bright -> pt1 to 0)
//   Set to val
// Rings
//  pos
//    Set to position
//    Move to center
//  brightness
//    set
//    pulse


// LASER
#[derive(Copy, Clone, Debug)]
pub enum TunnelgonBaseAnim {
    Pulse,
    SetToVal,
}

#[derive(Event, Clone)]
pub struct LaserAnimationEvent {
    pub affected_hexagons: Vec<HexagonDefinition>,
    pub base_anim: TunnelgonBaseAnim,
    pub indices: Vec<usize>,
    pub values: Vec<f32>,
}

signal_ids! {
    pub CancelAnim: CancelAnimData
}

#[derive(Clone, Default)]
pub struct CancelAnimData {
    pub indices: HashMap<HexagonDefinition, Vec<usize>>,
}

pub fn laser_animation_system(
    mut commands: Commands,
    query: Query<(Entity, &Tunnelgon, &Handle<TunnelgonMaterial>)>,
    mut event_reader: EventReader<LaserAnimationEvent>,
    mut reactors: ResMut<Reactors>,
    mut materials: ResMut<Assets<TunnelgonMaterial>>,
) {
    // Accumulate all events for this frame into single dataset
    let anim_data: Vec<(HexagonDefinition, TunnelgonBaseAnim, usize, f32)> = event_reader.read().flat_map(
        |ev| {
            let zipped_i_v: Vec<(usize, f32)> = ev.indices.iter().enumerate().map(|(i, index)| {
                (*index, *(ev.values.get(i).unwrap_or(&0f32)))
            }).collect();

            let mut out = vec![];
            for hex in ev.affected_hexagons.clone() {
                for (i, v) in zipped_i_v.clone() {
                    out.push((hex, ev.base_anim, i, v));
                }
            }
            out
        }
    ).collect();

    // Populate hashmap of hexagons -> indices to cancel and send it
    let signal = reactors.get_named::<CancelAnim>("cancel_tunnelgon_laser_anim");
    let mut cancel_anim_data = CancelAnimData::default();
    for (hex, _, i, _) in anim_data.iter() {
        let indices_for_hex = cancel_anim_data.indices.entry(*hex).or_insert(vec![*i]);
        indices_for_hex.push(*i);
    }
    signal.send(cancel_anim_data);

    for (hex, base_anim, laser_index, laser_value) in anim_data.iter().cloned() {
        for (entity, tg, tgm) in query.iter() {
            if hex != tg.hexagon_definition {
                continue;
            }
            let tgm_material = materials.get_mut(tgm).unwrap();
            if laser_index >= 8 { warn!("Got index out of range: {}", laser_index) }
            let laser_index = laser_index % 8;
            let entity_cloned = entity.clone();

            match base_anim {
                TunnelgonBaseAnim::SetToVal => {
                    tgm_material.params.laser[laser_index] = laser_value;
                }
                TunnelgonBaseAnim::Pulse => {
                    commands.spawn_task(move || async move {
                        let signal = world().named_signal::<CancelAnim>("cancel_tunnelgon_laser_anim");
                        let _ = signal.poll().await;

                        let tunnelgon_entity = world().entity(entity_cloned);
                        let materials = world().resource::<Assets<TunnelgonMaterial>>();

                        // Spawn PT1 anim
                        let pt1_entity = world().spawn_bundle(
                            Pt1Anim {
                                val: laser_value.clone(),
                                target: 0.,
                                time_constant: 0.03,
                            }
                        ).await.id();
                        let pt1_component = world().entity(pt1_entity).component::<Pt1Anim>();

                        let mat_handle = tunnelgon_entity.component::<Handle<TunnelgonMaterial>>()
                            .get(|mat_handle| mat_handle.clone()).await.unwrap();

                        // While the PT1 is still going, update the material
                        loop {
                            // Check if animation is meant to cancel
                            if let Some(cancel_data) = signal.try_read() {
                                if let Some(indices) = cancel_data.indices.get(&hex) {
                                    if indices.contains(&laser_index) {
                                        break;
                                    }
                                }
                            }

                            let (next_val, finished) = pt1_component.get(|pt1anim| { (pt1anim.get_val(), pt1anim.target_reached()) }).await.unwrap_or((0., true));
                            let mat_handle_cloned = mat_handle.clone();
                            let _ = materials.set(move |mut materials| {
                                let mut mat = materials.get_mut(mat_handle_cloned).unwrap();
                                mat.params.laser[laser_index] = next_val
                            }).await.unwrap();
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
    }
}

// RINGS
#[derive(Copy, Clone)]
pub enum RingBasePosAnim {
    SetToPosition,
    SlideLinear,
}

#[derive(Copy, Clone)]
pub enum RingBaseValAnim {
    SetToVal,
    Pulse,
}

#[derive(Event, Clone)]
pub struct RingAnimationEvent {
    pub affected_hexagons: Vec<HexagonDefinition>,
    pub base_pos_anim: RingBasePosAnim,
    pub base_val_anim: RingBaseValAnim,
    pub indices: Vec<usize>,
    pub positions_from: Vec<f32>,
    pub positions_to: Vec<f32>,
    pub values: Vec<f32>,
}

pub fn ring_animation_system(
    mut commands: Commands,
    query: Query<(Entity, &Tunnelgon, &Handle<TunnelgonMaterial>)>,
    mut event_reader: EventReader<RingAnimationEvent>,
    mut reactors: ResMut<Reactors>,
    mut materials: ResMut<Assets<TunnelgonMaterial>>,
) {
    // Accumulate all events for this frame into single dataset
    let anim_data: Vec<(HexagonDefinition, RingBasePosAnim, RingBaseValAnim, usize, f32, f32, f32)> = event_reader.read().flat_map(
        |ev| {
            let zipped_i_v: Vec<(usize, f32, f32, f32)> = ev.indices.iter().enumerate().map(|(i, index)| {
                (*index, *(ev.values.get(i).unwrap_or(&0f32)), *(ev.positions_from.get(i).unwrap_or(&0f32)), *(ev.positions_to.get(i).unwrap_or(&0f32)))
            }).collect();

            let mut out = vec![];
            for hex in ev.affected_hexagons.clone() {
                for (i, v, from, to) in zipped_i_v.clone() {
                    out.push((hex, ev.base_pos_anim, ev.base_val_anim, i, v, from, to));
                }
            }
            out
        }
    ).collect();

    // Populate hashmap of hexagons -> indices to cancel and send it
    let signal = reactors.get_named::<CancelAnim>("cancel_tunnelgon_ring_anim");
    let mut cancel_anim_data = CancelAnimData::default();
    for (hex, _, _, i, _, _, _) in anim_data.iter() {
        let indices_for_hex = cancel_anim_data.indices.entry(*hex).or_insert(vec![*i]);
        indices_for_hex.push(*i);
    }
    signal.send(cancel_anim_data);

    for (hex, base_pos_anim, base_val_anim, ring_index, ring_value, ring_pos_from, ring_pos_to) in anim_data.iter().cloned() {
        for (entity, tg, tgm) in query.iter() {
            if hex != tg.hexagon_definition {
                continue;
            }
            let tgm_material = materials.get_mut(tgm).unwrap();

            if ring_index.clone() >= 8 { warn!("Got index out of range: {}", ring_index) }
            let ring_index = ring_index.clone() % 8;
            let entity_cloned = entity.clone();

            match base_pos_anim {
                RingBasePosAnim::SetToPosition => { tgm_material.params.rings_pos[ring_index] = ring_pos_to; }
                RingBasePosAnim::SlideLinear => {
                    commands.spawn_task(move || async move {
                        let signal = world().named_signal::<CancelAnim>("cancel_tunnelgon_ring_anim");
                        let _ = signal.poll().await;

                        let tunnelgon_entity = world().entity(entity_cloned);
                        let materials = world().resource::<Assets<TunnelgonMaterial>>();

                        // Spawn Linear anim
                        let anim_entity = world().spawn_bundle(
                            LinearAnim {
                                val: ring_pos_from,
                                target: ring_pos_to,
                                speed: 1.,
                            }
                        ).await.id();
                        let anim_component = world().entity(anim_entity).component::<LinearAnim>();

                        let mat_handle = tunnelgon_entity.component::<Handle<TunnelgonMaterial>>()
                            .get(|mat_handle| mat_handle.clone()).await.unwrap();

                        // While the Anim is still going, update the material
                        loop {
                            // Check if animation is meant to cancel
                            if let Some(cancel_data) = signal.try_read() {
                                if let Some(indices) = cancel_data.indices.get(&hex) {
                                    if indices.contains(&ring_index) {
                                        break;
                                    }
                                }
                            }

                            let (next_val, finished) = anim_component.get(|linear_anim| { (linear_anim.get_val(), linear_anim.target_reached()) }).await.unwrap_or((0., true));
                            let mat_handle_cloned = mat_handle.clone();
                            let _ = materials.set(move |mut materials| {
                                let mut mat = materials.get_mut(mat_handle_cloned).unwrap();
                                mat.params.rings_pos[ring_index] = next_val
                            }).await.unwrap();
                            if finished {
                                break;
                            }
                        }

                        world().entity(anim_entity).despawn().await;

                        Ok(())
                    });
                }
            }

            match base_val_anim {
                RingBaseValAnim::SetToVal => { tgm_material.params.rings_amp[ring_index] = ring_value; }
                RingBaseValAnim::Pulse => {
                    commands.spawn_task(move || async move {
                        let signal = world().named_signal::<CancelAnim>("cancel_tunnelgon_ring_anim");
                        let _ = signal.poll().await;

                        let tunnelgon_entity = world().entity(entity_cloned);
                        let materials = world().resource::<Assets<TunnelgonMaterial>>();

                        // Spawn Linear anim
                        let anim_entity = world().spawn_bundle(
                            Pt1Anim {
                                val: ring_value,
                                target: 0.,
                                ..default()
                            }
                        ).await.id();
                        let anim_component = world().entity(anim_entity).component::<Pt1Anim>();

                        let mat_handle = tunnelgon_entity.component::<Handle<TunnelgonMaterial>>()
                            .get(|mat_handle| mat_handle.clone()).await.unwrap();

                        // While the Anim is still going, update the material
                        loop {
                            // Check if animation is meant to cancel
                            if let Some(cancel_data) = signal.try_read() {
                                if let Some(indices) = cancel_data.indices.get(&hex) {
                                    if indices.contains(&ring_index) {
                                        break;
                                    }
                                }
                            }

                            let (next_val, finished) = anim_component.get(|pt1anim| { (pt1anim.get_val(), pt1anim.target_reached()) }).await.unwrap_or((0., true));
                            let mat_handle_cloned = mat_handle.clone();
                            let _ = materials.set(move |mut materials| {
                                let mut mat = materials.get_mut(mat_handle_cloned).unwrap();
                                mat.params.rings_amp[ring_index] = next_val
                            }).await.unwrap();
                            if finished {
                                break;
                            }
                        }

                        world().entity(anim_entity).despawn().await;

                        Ok(())
                    });
                }
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct TunnelgonAccum {
    pub(crate) enabled: bool,
}

pub fn tunnelgon_accum(
    mut materials: ResMut<Assets<TunnelgonMaterial>>,
    mut beat_reader: EventReader<BeatEvent>,
    mut query: Query<&Handle<TunnelgonMaterial>>,
    time: Res<Time<Real>>,
    settings: Res<TunnelgonAccum>,
) {
    for _ in beat_reader.read() {
        if settings.enabled {
            for mat_handle in query.iter() {
                if let Some(mut mat) = materials.get_mut(mat_handle) {
                    mat.params.tun_accum_target = mat.params.tun_accum_target + 1.;
                }
            }
        }
    }

    for mat_handle in query.iter() {
        if let Some(mut mat) = materials.get_mut(mat_handle) {
            mat.params.tun_accum = mat.params.tun_accum + (mat.params.tun_accum_target - mat.params.tun_accum) * (time.delta_seconds() / (time.delta_seconds() + 0.1));
            println!("{} {}", mat.params.tun_accum, mat.params.tun_accum_target);
        }
    }
}