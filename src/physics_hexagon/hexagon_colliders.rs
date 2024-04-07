use std::f32::consts::PI;
use bevy::prelude::{BuildChildren, Commands, Entity, Quat, SpatialBundle, Transform, Component};
use bevy_rapier3d::prelude::{Ccd, Collider, RigidBody};
use crate::hexagon::HexagonDefinition;

#[derive(Component)]
struct HexagonCollider {
    hexagon_definition: HexagonDefinition,
}

pub fn spawn_hexagon_collier(
    commands: &mut Commands,
    // Defines the size of the hexagon
    hexagon_definition: HexagonDefinition,
    // Height from floor to ceiling cap
    height: f32,
    // Added to radius
    wall_width: f32,
    floor_thickness: f32,
) -> Entity {

    // Outer radius of hexagon, which will align with the inside of the walls
    let radius = hexagon_definition.size().x / 2.;
    let inner_radius = 3_f32.sqrt() / 2. * radius;

    let hexagon_collider_entity = commands.spawn((
        HexagonCollider { hexagon_definition },
        SpatialBundle::default(),
    )).id();

    let hexagon_elements = [
        spawn_wall(commands, &radius, height, &wall_width, 0),
        spawn_wall(commands, &radius, height, &wall_width, 1),
        spawn_wall(commands, &radius, height, &wall_width, 2),
        spawn_wall(commands, &radius, height, &wall_width, 3),
        spawn_wall(commands, &radius, height, &wall_width, 4),
        spawn_wall(commands, &radius, height, &wall_width, 5),
        spawn_floor(commands, &inner_radius, &floor_thickness, 0),
        spawn_floor(commands, &inner_radius, &floor_thickness, 1),
        spawn_floor(commands, &inner_radius, &floor_thickness, 2),
    ];

    commands.entity(hexagon_collider_entity).push_children(&hexagon_elements);

    hexagon_collider_entity
}


fn spawn_wall(
    commands: &mut Commands,
    size: &f32,
    height: f32,
    wall_width: &f32,
    index: isize,
) -> Entity {
    let mut x = 0.;
    let mut y = 3_f32.sqrt() / 2. * size.clone() + wall_width / 2.;

    let angle = (index as f32) * PI / 3.;

    let x_rot = x.clone() * angle.cos() - y.clone() * angle.sin();
    let y_rot = x.clone() * angle.sin() + y.clone() * angle.cos();

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            x_rot,
            y_rot,
            0.0,
        ).with_rotation(Quat::from_rotation_z(angle))
        ),
        RigidBody::Fixed,
        Ccd::enabled(),
        Collider::cuboid(size.clone() / 2., wall_width.clone() / 2., height / 2.),
    )).id()
}

fn spawn_floor(
    commands: &mut Commands,
    size: &f32,
    floor_thickness: &f32,
    index: isize,
) -> Entity {
    let angle = (index as f32) * PI / 3.;

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            0.0,
            0.0,
            -floor_thickness.clone() / 2.,
        ).with_rotation(Quat::from_rotation_z(angle))
        ),
        RigidBody::Fixed,
        Ccd::enabled(),
        Collider::cuboid(size.clone() / 3_f32.sqrt(), size.clone(), floor_thickness.clone() / 2.),
    )).id()
}