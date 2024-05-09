use std::f32::consts::PI;
use std::future::Future;
use std::process::Command;
use std::ptr::read;
use bevy::asset::Assets;
use bevy::math::Quat;
use bevy::prelude::{Asset, Color, ColorMaterial, Commands, Component, default, DespawnRecursiveExt, Entity, Event, EventReader, Handle, Image, Mesh, Query, RegularPolygon, Res, ResMut, Transform, TypePath, With};
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle};
use bevy_defer::{async_system, AsyncAccess, AsyncCommandsExtension, signal_ids, world};
use bevy_defer::reactors::Reactors;
use bevy_defer::signals::{Receiver, Sender, Signal, Signals, SignalSender};
use crate::anim::{ParameterAnimation, Pt1Anim};
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
    mut reactors: ResMut<Reactors>
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
    SetToVal
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
    query: Query<(Entity, &Tunnelgon)>,
    mut event_reader: EventReader<LaserAnimationEvent>,
    mut reactors: ResMut<Reactors>,
) {
    for ev in event_reader.read() {
        let signal = reactors.get_named::<CancelAnim>("cancel_tunnelgon_laser_anim");
        signal.send(ev.indices.clone());
        for (entity, tg) in query.iter() {
            if !ev.affected_hexagons.contains(&tg.hexagon_definition) {
                continue;
            }
            for (i, li) in ev.indices.iter().enumerate() {
                let laser_index = li.clone();
                let laser_value = ev.values.get(i).unwrap_or(&0.).clone();
                let entity_cloned = entity.clone();

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
                            ..default()
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