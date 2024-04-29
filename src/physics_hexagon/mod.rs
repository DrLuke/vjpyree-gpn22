mod hexagon_colliders;
pub mod render;
pub mod effectors;
pub mod lights;

use std::f32::consts::PI;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget};
use bevy::render::view::RenderLayers;
use bevy_rapier3d::prelude::{Ccd, Collider, RigidBody};
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::effectors::EffectorsPlugin;
use crate::physics_hexagon::hexagon_colliders::spawn_hexagon_collier;
use crate::physics_hexagon::lights::{spawn_led_tubes};
use crate::physics_hexagon::lights::animations::wave_animation_system;
use crate::physics_hexagon::lights::physical_lights::{drive_lights_system, HexagonLights, PhysicalLedTube, PhysicalLedTubeLed, PhysicalTubeIndex, spawn_physical_leds};
use crate::physics_hexagon::render::PhysicsHexagonRenderTarget;
use crate::propagating_render_layers::PropagatingRenderLayers;

pub struct PhysicsHexagonPlugin;

impl Plugin for PhysicsHexagonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EffectorsPlugin));
        app.init_resource::<PhysicsHexagonRenderTarget>();
        app.add_systems(Startup, (
            init_physics_hexagons,
            spawn_led_tubes.after(init_physics_hexagons),
            spawn_physical_leds.after(spawn_led_tubes)
        ));
        app.add_systems(Update, hexagon_physics_element_cleanup_system);
        app.add_systems(Update, (drive_lights_system, wave_animation_system));
        app.register_type::<PhysicalTubeIndex>();
        app.register_type::<PhysicalLedTube>();
        app.register_type::<PhysicalLedTubeLed>();
    }
}

fn init_physics_hexagons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rt: Res<PhysicsHexagonRenderTarget>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 0.77, // Experimentally determined
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 1500.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            camera: Camera {
                order: -100,
                target: RenderTarget::Image(rt.render_target.clone()),
                clear_color: Color::rgba(0., 0., 0., 0.).into(),
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..Camera3dBundle::default()
        },
        BloomSettings::NATURAL,
        PropagatingRenderLayers { render_layers: RenderLayers::from_iter(1..2) },
    ));


    spawn_physics_hexagon(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::Main);
}


#[derive(Component)]
pub struct PhysicsHexagon {
    pub hexagon_definition: HexagonDefinition,
}

/// All colliders and hexagon meshes should be child entities of this
#[derive(Component)]
pub struct HexagonGeometry;

fn spawn_physics_hexagon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    hexagon_definition: HexagonDefinition,
) -> Entity {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.8, 0.8, 0.8, 1.),
        ..default()
    });

    let radius = hexagon_definition.size().x / 2.;
    let wall_width = 10.;
    let wall_height = 500.;
    let inner_radius = 3_f32.sqrt() / 2. * radius;
    let floor_thickness = 10.;
    let size_scale = 0.95; // Scale down 3d hexagons so they fit into 2d footprint

    let wall = meshes.add(Cuboid::new(radius.clone(), wall_width.clone(), wall_height));
    let floor = meshes.add(Cuboid::new(radius.clone(), inner_radius.clone() * 2., floor_thickness.clone()));

    let hexagon_entity = commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            hexagon_definition.center().x - 1920. / 2.,
            hexagon_definition.center().y - 1080. / 2.,
            0.,
        )),
        PhysicsHexagon { hexagon_definition },
        PropagatingRenderLayers { render_layers: RenderLayers::layer(1) },
    )
    )
        .with_children(|mut child_builder| {
            child_builder.spawn((
                HexagonLights {},
                SpatialBundle::default(),
            ));
        })
        .id();


    let hexagon_geometry = commands.spawn((
        HexagonGeometry {},
        SpatialBundle::from_transform(Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(size_scale)))
    )).id();

    let hexagon_elements = [
        spawn_hexagon_collier(commands, hexagon_definition, wall_height, wall_width, floor_thickness),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 0),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 1),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 2),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 3),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 4),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 5),
        spawn_floor(commands, &floor, &material, &floor_thickness, 0),
        spawn_floor(commands, &floor, &material, &floor_thickness, 1),
        spawn_floor(commands, &floor, &material, &floor_thickness, 2),
    ];

    commands.entity(hexagon_geometry).push_children(&hexagon_elements);
    commands.entity(hexagon_entity).push_children(&[hexagon_geometry]);

    let eye_01: Handle<Mesh> = asset_server.load("eye_01.glb#Mesh0/Primitive0");
    let eye_01_material: Handle<StandardMaterial> = asset_server.load("eye_01.glb#Material0");

    for n in 1..10 {
        let entity = commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(
                    0.,
                    n.clone() as f32,
                    100. + n as f32 * 100.),
                ..default()
            },
            HexagonPhysicsElement,
            RigidBody::Dynamic,
            Ccd::enabled(),
            Collider::ball(50.),
            PropagatingRenderLayers { render_layers: RenderLayers::layer(1) },
        )).id();

        let eye_mesh = commands.spawn((
            PbrBundle {
                mesh: eye_01.clone(),
                material: eye_01_material.clone(),
                transform: Transform::from_scale(Vec3::new(4000., 4000., 4000.)),
                ..default()
            }
        )).id();
        commands.entity(hexagon_entity).push_children(&[entity]);
        commands.entity(entity).push_children(&[
            eye_mesh
        ]);
    }

    /*commands
        .spawn((PointLightBundle {
            transform: Transform::from_xyz(hexagon_definition.center().x - 1920. / 2., hexagon_definition.center().y - 1080. / 2., 100.0),
            point_light: PointLight {
                intensity: 100_000_000.0,
                range: 40_000.0,
                radius: 100.,
                color: Color::WHITE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        }, PropagatingRenderLayers { render_layers: RenderLayers::layer(1) }));*/

    commands
        .spawn((PointLightBundle {
            transform: Transform::from_xyz(hexagon_definition.center().x - 1920. / 2. + 100., hexagon_definition.center().y - 1080. / 2., 150.0),
            point_light: PointLight {
                intensity: 300_000_000.0,
                range: 40_000.0,
                radius: 100.,
                color: Color::RED,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        }, PropagatingRenderLayers { render_layers: RenderLayers::layer(1) }));
    commands
        .spawn((PointLightBundle {
            transform: Transform::from_xyz(hexagon_definition.center().x - 1920. / 2. - 100., hexagon_definition.center().y - 1080. / 2., 150.0),
            point_light: PointLight {
                intensity: 300_000_000.0,
                range: 40_0000.0,
                radius: 100.,
                color: Color::BLUE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        }, PropagatingRenderLayers { render_layers: RenderLayers::layer(1) }));

    return hexagon_entity;
}

fn spawn_wall(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    size: &f32,
    wall_width: &f32,
    index: isize,
) -> Entity {
    let x = 0.;
    let y = 3_f32.sqrt() / 2. * size.clone() + wall_width / 2.;

    let angle = (index as f32) * PI / 3.;

    let x_rot = x.clone() * angle.cos() - y.clone() * angle.sin();
    let y_rot = x.clone() * angle.sin() + y.clone() * angle.cos();

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(
                x_rot,
                y_rot,
                0.0,
            ).with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        },
    )).id()
}

fn spawn_floor(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    floor_thickness: &f32,
    index: isize,
) -> Entity {
    let angle = (index as f32) * PI / 3.;

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            transform: Transform::from_xyz(
                0.0,
                0.0,
                -floor_thickness.clone() / 2.,
            ).with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        },
    )).id()
}

/// Tag component for physics objects that are spawned into hexagons that shall be manipulated by effectors
#[derive(Component, Default, Copy, Clone)]
pub struct HexagonPhysicsElement;

/// Clean up all physics element that dropped out of the Hexagon
fn hexagon_physics_element_cleanup_system(
    mut commands: Commands,
    query: Query<(Entity, &Parent, &Transform), With<HexagonPhysicsElement>>,
    parent_query: Query<&PhysicsHexagon>,
) {
    for (entity, parent, transform) in query.iter() {
        let hexagon_definition = match parent_query.get(parent.get()) {
            Ok(phyics_hexagon) => { phyics_hexagon.hexagon_definition }
            Err(_) => { HexagonDefinition::Main }
        };

        if transform.translation.truncate().length() > (hexagon_definition.size().x * 1.1 / 2.) {
            commands.entity(entity).despawn_recursive();
        }

        if transform.translation.z.abs() > 10000. {
            commands.entity(entity).despawn_recursive();
        }
    }
}