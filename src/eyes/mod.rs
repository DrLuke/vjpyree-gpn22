use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier3d::prelude::{Ccd, Collider, RigidBody};

pub struct EyesPlugin;

impl Plugin for EyesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (eyes_init));
    }
}

fn eyes_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(1.),
            near: 2000.,
            far: -2000.,
            ..default()
        }),
        ..Camera3dBundle::default()
    });


    spawn_eyes(&mut commands, &mut meshes, &mut materials, 300.);
}


#[derive(Component)]
pub struct EyesHexagon;

fn spawn_eyes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // Outer radius of hexagon
    radius: f32,
) -> Entity {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.8, 0.8, 0.8, 1.),
        ..default()
    });

    let wall_width = 50.;
    let inner_radius = 3_f32.sqrt() / 2. * radius;
    let floor_thickness = 10.;

    let wall = meshes.add(Cuboid::new(radius.clone(), wall_width.clone(), 10.));
    let floor = meshes.add(Cuboid::new(radius.clone(), inner_radius.clone() * 2., floor_thickness.clone()));

    let hexagon_entity = commands.spawn((
        SpatialBundle::default(),
        EyesHexagon {},
    )
    ).id();

    let hexagon_elements = [
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 0),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 1),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 2),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 3),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 4),
        spawn_wall(commands, &wall, &material, &radius, &wall_width, 5),
        spawn_floor(commands, &floor, &material, &inner_radius, &floor_thickness, 0),
        spawn_floor(commands, &floor, &material, &inner_radius, &floor_thickness, 1),
        spawn_floor(commands, &floor, &material, &inner_radius, &floor_thickness, 2),
    ];

    commands.entity(hexagon_entity).push_children(&hexagon_elements);


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
        RigidBody::Fixed,
        Ccd::enabled(),
        Collider::cuboid(size.clone() / 2., wall_width.clone() / 2., 10. / 2.),
    )).id()
}

fn spawn_floor(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    size: &f32,
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
                -floor_thickness.clone()/2.,
            ).with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        },
        RigidBody::Fixed,
        Ccd::enabled(),
        Collider::cuboid(size.clone() / 3_f32.sqrt(), size.clone() , floor_thickness.clone() / 2.),
    )).id()
}