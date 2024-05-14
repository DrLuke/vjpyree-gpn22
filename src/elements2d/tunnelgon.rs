use std::f32::consts::PI;
use std::future::Future;
use std::process::Command;
use std::ptr::read;
use bevy::asset::Assets;
use bevy::log::warn;
use bevy::math::Quat;
use bevy::prelude::{Asset, Color, ColorMaterial, Commands, Component, default, DespawnRecursiveExt, Entity, Event, EventReader, Handle, Image, Mesh, Query, RegularPolygon, Res, ResMut, Transform, TypePath, With};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_defer::{async_system, AsyncAccess, AsyncCommandsExtension, signal_ids, world};
use bevy_defer::reactors::Reactors;
use bevy_defer::signals::{Receiver, Sender, Signal, Signals, SignalSender};
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
#[derive(Clone)]
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
    pub CancelAnim: Vec<usize>
}

pub fn laser_animation_system(
    mut commands: Commands,
    query: Query<(Entity, &Tunnelgon, &Handle<TunnelgonMaterial>)>,
    mut event_reader: EventReader<LaserAnimationEvent>,
    mut reactors: ResMut<Reactors>,
    mut materials: ResMut<Assets<TunnelgonMaterial>>,
) {
    for ev in event_reader.read() {
        let signal = reactors.get_named::<CancelAnim>("cancel_tunnelgon_laser_anim");
        signal.send(ev.indices.clone());
        for (entity, tg, tgm) in query.iter() {
            if !ev.affected_hexagons.contains(&tg.hexagon_definition) {
                continue;
            }
            let tgm_material = materials.get_mut(tgm).unwrap();
            for (i, li) in ev.indices.iter().enumerate() {
                if li.clone() >= 8 { warn!("Got index out of range: {}", li) }
                let laser_index = li.clone() % 8;
                let laser_value = ev.values.get(i).unwrap_or(&0.).clone();
                let entity_cloned = entity.clone();

                match ev.base_anim {
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
                                if let Some(cancel_indices) = signal.try_read() {
                                    if cancel_indices.contains(&laser_index) {
                                        break;
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
}

// RINGS
#[derive(Clone)]
pub enum RingBasePosAnim {
    SetToPosition,
    SlideLinear,
}

#[derive(Clone)]
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
    for ev in event_reader.read() {
        let signal = reactors.get_named::<CancelAnim>("cancel_tunnelgon_ring_anim");
        signal.send(ev.indices.clone());
        for (entity, tg, tgm) in query.iter() {
            if !ev.affected_hexagons.contains(&tg.hexagon_definition) {
                continue;
            }
            let tgm_material = materials.get_mut(tgm).unwrap();
            for (i, li) in ev.indices.iter().enumerate() {
                if li.clone() >= 8 { warn!("Got index out of range: {}", li) }
                let ring_index = li.clone() % 8;
                let ring_value = ev.values.get(i).unwrap_or(&0.).clone();
                let ring_pos_from = ev.positions_from.get(i).unwrap_or(&0.).clone();
                let ring_pos_to = ev.positions_to.get(i).unwrap_or(&0.).clone();
                let entity_cloned = entity.clone();

                match ev.base_pos_anim {
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
                                    speed: 1.
                                }
                            ).await.id();
                            let anim_component = world().entity(anim_entity).component::<LinearAnim>();

                            let mat_handle = tunnelgon_entity.component::<Handle<TunnelgonMaterial>>()
                                .get(|mat_handle| mat_handle.clone()).await.unwrap();

                            // While the Anim is still going, update the material
                            loop {
                                // Check if animation is meant to cancel
                                if let Some(cancel_indices) = signal.try_read() {
                                    if cancel_indices.contains(&ring_index) {
                                        break;
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

                match ev.base_val_anim {
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
                                if let Some(cancel_indices) = signal.try_read() {
                                    if cancel_indices.contains(&ring_index) {
                                        break;
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
}