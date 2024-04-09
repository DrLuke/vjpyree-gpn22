mod hexagon_colliders;
mod fix_perspective;
mod render;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier3d::prelude::{Ccd, Collider, RigidBody};
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::fix_perspective::{fix_perspective_system, FixPerspectiveSubject, FixPerspectiveTarget};
use crate::physics_hexagon::hexagon_colliders::spawn_hexagon_collier;
use crate::physics_hexagon::render::PhysicsHexagonRenderTarget;

pub struct PhysicsHexagonPlugin;

impl Plugin for PhysicsHexagonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsHexagonRenderTarget>();
        app.add_systems(Startup, (eyes_init));
        app.add_systems(Update, (fix_perspective_system));
    }
}

fn eyes_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rt: Res<PhysicsHexagonRenderTarget>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 30_f32.to_radians(),
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 2000.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            camera: Camera {
                order: -100,
                target: RenderTarget::Image(rt.render_target.clone()),
                clear_color: Color::BLACK.into(),
                ..default()
            },
            ..Camera3dBundle::default()
        },
        FixPerspectiveTarget {},
    ));


    spawn_eyes(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::Main);
    spawn_eyes(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::A1);
    spawn_eyes(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::B1);
    spawn_eyes(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::A2);
    spawn_eyes(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::B2);
    spawn_eyes(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::A3);
    spawn_eyes(&mut commands, &mut meshes, &mut materials, &asset_server, HexagonDefinition::B3);
}


#[derive(Component)]
pub struct PhysicsHexagon;

/// All colliders and hexagon meshes should be child entities of this
#[derive(Component)]
pub struct HexagonGeometry;

fn spawn_eyes(
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
    let wall_height = 1000.;
    let inner_radius = 3_f32.sqrt() / 2. * radius;
    let floor_thickness = 10.;

    let wall = meshes.add(Cuboid::new(radius.clone(), wall_width.clone(), wall_height));
    let floor = meshes.add(Cuboid::new(radius.clone(), inner_radius.clone() * 2., floor_thickness.clone()));

    let hexagon_entity = commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            hexagon_definition.center().x - 1920. / 2.,
            hexagon_definition.center().y - 1080. / 2.,
            0.,
        )),
        PhysicsHexagon {},
        FixPerspectiveSubject {
            original_transform: Transform::from_xyz(
                hexagon_definition.center().x - 1920. / 2.,
                hexagon_definition.center().y - 1080. / 2.,
                0.0,
            )
        }
    )
    ).id();

    let hexagon_geometry = commands.spawn((
        HexagonGeometry {},
        SpatialBundle::from_transform(Transform::from_xyz(0., 0., 500.))
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
                    hexagon_definition.center().x - 1920. / 2.,
                    hexagon_definition.center().y - 1080. / 2. + n.clone() as f32,
                    100. + n as f32 * 100.),
                ..default()
            },
            RigidBody::Dynamic,
            Ccd::enabled(),
            Collider::ball(50.),
        )).id();

        let eye_mesh = commands.spawn((
            PbrBundle {
                mesh: eye_01.clone(),
                material: eye_01_material.clone(),
                transform: Transform::from_scale(Vec3::new(4000., 4000., 4000.)),
                ..default()
            }
        )).id();
        commands.entity(entity).push_children(&[
            eye_mesh
        ]);
    }

    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(hexagon_definition.center().x - 1920. / 2., hexagon_definition.center().y - 1080. / 2., 100.0),
            point_light: PointLight {
                intensity: 100_000_000.0,
                range: 40_000.0,
                color: Color::WHITE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });

    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(hexagon_definition.center().x - 1920. / 2. + 100., hexagon_definition.center().y - 1080. / 2., 150.0),
            point_light: PointLight {
                intensity: 300_000_000.0,
                range: 40_000.0,
                color: Color::RED,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(hexagon_definition.center().x - 1920. / 2. - 100., hexagon_definition.center().y - 1080. / 2., 150.0),
            point_light: PointLight {
                intensity: 300_000_000.0,
                range: 40_0000.0,
                color: Color::BLUE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });

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
    let mut x = 0.;
    let mut y = 3_f32.sqrt() / 2. * size.clone() + wall_width / 2.;

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