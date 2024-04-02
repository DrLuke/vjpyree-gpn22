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
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        /*projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(1.),
            near: 2000.,
            far: -2000.,
            ..default()
        }),*/
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 30_f32.to_radians(),
            ..default()
        }),
        transform: Transform::from_xyz(0., 0., 2000.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
        ..Camera3dBundle::default()
    });


    spawn_eyes(&mut commands, &mut meshes, &mut materials, 300., &asset_server);
}


#[derive(Component)]
pub struct EyesHexagon;

fn spawn_eyes(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // Outer radius of hexagon
    radius: f32,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let material = materials.add(StandardMaterial {
        base_color: Color::rgba(0.8, 0.8, 0.8, 1.),
        ..default()
    });

    let wall_width = 50.;
    let inner_radius = 3_f32.sqrt() / 2. * radius;
    let floor_thickness = 10.;

    let wall = meshes.add(Cuboid::new(radius.clone(), wall_width.clone(), 1000.));
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

    let eye_01: Handle<Mesh> = asset_server.load("eye_01.glb#Mesh0/Primitive0");
    let eye_01_material: Handle<StandardMaterial> = asset_server.load("eye_01.glb#Material0");

    for n in 1..10 {
        let entity = commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0., n.clone() as f32, 100. + n as f32 * 100.),
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
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            point_light: PointLight {
                intensity: 100_000_000.0,
                range: 10_000.0,
                color: Color::WHITE,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });

    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(100.0, 0.0, 150.0),
            point_light: PointLight {
                intensity: 300_000_000.0,
                range: 10_000.0,
                color: Color::RED,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(-100.0, 0.0, 150.0),
            point_light: PointLight {
                intensity: 300_000_000.0,
                range: 10_000.0,
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
        RigidBody::Fixed,
        Ccd::enabled(),
        Collider::cuboid(size.clone() / 2., wall_width.clone() / 2., 1000. / 2.),
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
                -floor_thickness.clone() / 2.,
            ).with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        },
        RigidBody::Fixed,
        Ccd::enabled(),
        Collider::cuboid(size.clone() / 3_f32.sqrt(), size.clone(), floor_thickness.clone() / 2.),
    )).id()
}