use std::f32::consts::PI;
use bevy::math::Vec2;
use bevy::pbr::{PointLightBundle, SpotLightBundle};
use bevy::prelude::{BuildChildren, Children, Color, Commands, Component, Entity, error, PointLight, Quat, Query, Reflect, SpatialBundle, SpotLight, Transform, With};
use bevy::render::view::RenderLayers;
use bevy::tasks::futures_lite::StreamExt;
use bevy::utils::default;
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::lights::led_tube::{LEDS_COUNT, LedTube, LedTubeLed, TUBE_LENGTH, TubeIndex};
use crate::physics_hexagon::PhysicsHexagon;
use crate::propagating_render_layers::PropagatingRenderLayers;

const EDGE_LENGTH: f32 = 340.;
// 365. is the actual length, but it's a bit shortened for less edge overlap
const EDGE_LED_COUNT: i32 = 32;
const EDGE_LED_DIVISOR: i32 = 4;
const ACTUAL_EDGE_LED_COUNT: i32 = EDGE_LED_COUNT / EDGE_LED_DIVISOR;
const LIGHTS_PER_TUBE: i32 = 16 / EDGE_LED_DIVISOR;

/// Root component for all the lights of a hexagon
#[derive(Component, Reflect)]
pub struct HexagonLights;

#[derive(Copy, Clone, Reflect, Eq, PartialEq)]
pub enum PhysicalTubeIndex {
    AB,
    BC,
    CD,
    DE,
    EF,
    FA,
}

impl From<i32> for PhysicalTubeIndex {
    fn from(value: i32) -> Self {
        match value {
            0 => { Self::AB }
            1 => { Self::BC }
            2 => { Self::CD }
            3 => { Self::DE }
            4 => { Self::EF }
            5 => { Self::FA }
            _ => { Self::AB }
        }
    }
}

impl PhysicalTubeIndex {
    pub fn get_angle(&self) -> f32 {
        match self {
            PhysicalTubeIndex::AB => { PI / 3. }
            PhysicalTubeIndex::BC => { 0. }
            PhysicalTubeIndex::CD => { -PI / 3. }
            PhysicalTubeIndex::DE => { -2. * PI / 3. }
            PhysicalTubeIndex::EF => { -PI }
            PhysicalTubeIndex::FA => { -4. * PI / 3. }
        }
    }
}

/// Represents the physical lights of each LED tube inside the hexagon
#[derive(Component, Reflect)]
pub struct PhysicalLedTube {
    index: PhysicalTubeIndex,
}

#[derive(Component, Reflect)]
pub struct PhysicalLedTubeLed {
    /// Light index, from 0 to 191 (6*32 LEDs). Counting starts from the leftmost corner and goes clockwise
    index: isize,
    /// The LedTubeLeds this light is driven by. If it's more than one it'll average all the values.
    led_tube_leds: Vec<Entity>,
}

pub fn spawn_physical_leds(
    mut commands: Commands,
    physical_hexagon_query: Query<(Entity, &Children, &PhysicsHexagon)>,
    hexagon_lights_query: Query<Entity, With<HexagonLights>>,
    led_tube_query: Query<(&LedTube, &Children)>,
    led_tube_led_query: Query<&LedTubeLed>,
) {
    let Some((physics_hexagon_entity, physics_hexagon_children, physics_hexagon)) = physical_hexagon_query
        .iter()
        .find(|(_, _, ph)| {
            ph.hexagon_definition == HexagonDefinition::Main
        }) else {
        error!("Hexagon for definition {:?} doesn't exist!", HexagonDefinition::Main);
        return;
    };

    let Some(hexagon_lights_entity) = physics_hexagon_children
        .iter()
        .map(|child| { (*child).clone() })
        .find(|child| {
            hexagon_lights_query.contains(*child)
        }) else {
        error!("Hexagon {:?} has no HexagonLights child", HexagonDefinition::Main);
        return;
    };

    for i in 0..6 {
        let rotation = PhysicalTubeIndex::from(i).get_angle();
        let tube_entity = commands.spawn((
            PhysicalLedTube {
                index: PhysicalTubeIndex::from(i)
            },
            SpatialBundle {
                transform: Transform::from_rotation(Quat::from_rotation_z(rotation)),
                ..default()
            },
        )).id();
        commands.entity(hexagon_lights_entity).push_children(&[tube_entity]);

        for j in 0..ACTUAL_EDGE_LED_COUNT {
            let offset = ((j as f32 + 0.5) / ACTUAL_EDGE_LED_COUNT as f32) * EDGE_LENGTH - EDGE_LENGTH / 2.;
            let led_tube_led_index = (i as isize * ACTUAL_EDGE_LED_COUNT as isize) + j as isize;
            let led_entity = commands.spawn((
                SpotLightBundle {
                    spot_light: SpotLight {
                        intensity: 500_000_000.0,
                        range: 3000.0,
                        radius: 5.,
                        color: Color::ORANGE_RED,
                        shadows_enabled: true,
                        outer_angle: PI / 4.,
                        inner_angle: PI / 6.,
                        ..default()
                    },
                    transform: Transform::from_xyz(offset, 300., 150.).with_rotation(Quat::from_rotation_x(-PI / 4.)),
                    ..default()
                },
                PhysicalLedTubeLed {
                    index: led_tube_led_index,
                    led_tube_leds: get_led_tube_led_entities(led_tube_led_index as i32, &led_tube_query, &led_tube_led_query),
                }
            )).id();
            commands.entity(tube_entity).push_children(&[led_entity]);
        }
    }
}

fn get_led_tube_led_entities(
    index: i32,
    led_tube_query: &Query<(&LedTube, &Children)>,
    led_tube_led_query: &Query<&LedTubeLed>,
) -> Vec<Entity> {
    let tube_index = map_index_to_tube_index(index);
    let Some((led_tube, led_tube_led_entities)) = led_tube_query
        .iter()
        .find(|(led_tube, children)| {
            led_tube.get_tube_index() == tube_index
        }) else {
        error!("Couldn't find LedTube for index {:?}", tube_index);
        return vec![];
    };

    let start_led_index = (index % LIGHTS_PER_TUBE) * LIGHTS_PER_TUBE;
    let end_index = start_led_index + EDGE_LED_DIVISOR; //Exclusive index
    let mut led_tube_led_matched_entities: Vec<Entity> = vec![];
    for led_tube_entity in led_tube_led_entities {
        let led_tube_led = led_tube_led_query.get(*led_tube_entity).unwrap();
        if (start_led_index..end_index).contains(&(led_tube_led.get_index() as i32)) {
            led_tube_led_matched_entities.push(*led_tube_entity);
        }
    };

    if led_tube_led_matched_entities.len() != EDGE_LED_DIVISOR as usize {
        error!("Matched LED count for index {} is incorrect: {} (expected: {})", index, led_tube_led_matched_entities.len(), EDGE_LED_DIVISOR);
    }

    led_tube_led_matched_entities
}

fn map_index_to_tube_index(
    index: i32,
) -> TubeIndex {
    if (00 * LIGHTS_PER_TUBE..01 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Six; };
    if (01 * LIGHTS_PER_TUBE..02 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Eight; };
    if (02 * LIGHTS_PER_TUBE..03 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Ten; };
    if (03 * LIGHTS_PER_TUBE..04 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Twelve; };
    if (04 * LIGHTS_PER_TUBE..05 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Fourteen; };
    if (05 * LIGHTS_PER_TUBE..06 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Sixteen; };
    if (06 * LIGHTS_PER_TUBE..07 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Seventeen; };
    if (07 * LIGHTS_PER_TUBE..08 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Fifteen; };
    if (08 * LIGHTS_PER_TUBE..09 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Thirteen; };
    if (09 * LIGHTS_PER_TUBE..10 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Eleven; };
    if (10 * LIGHTS_PER_TUBE..11 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Nine; };
    if (11 * LIGHTS_PER_TUBE..12 * LIGHTS_PER_TUBE).contains(&index) { return TubeIndex::Seven; };

    error!("Index {} couldn't be mapped, returning default", index);
    TubeIndex::Six
}

/// Update the lights by averaging the color values of all LEDs linked to them
pub fn drive_lights_system(
    mut lights_query: Query<(&PhysicalLedTubeLed, &mut SpotLight)>,
    led_query: Query<&LedTubeLed>,
) {
    for (phyiscal_led_tube, mut spotlight) in lights_query.iter_mut() {
        // Average the colors by adding the square of each component up,
        let color = phyiscal_led_tube.led_tube_leds.iter()
            .map(|led_tube_led_entity| { led_query.get(*led_tube_led_entity).unwrap() })
            .fold(Color::BLACK, |mut acc, led_tube_led| {
                acc + square_color(led_tube_led.color)
            });
        spotlight.color = sqrt_color(color * (1./LIGHTS_PER_TUBE as f32));
    }
}

fn square_color(color: Color) -> Color {
    Color::rgb(
        color.r().powf(2.),
        color.g().powf(2.),
        color.b().powf(2.),
    )
}

fn sqrt_color(color: Color) -> Color {
    Color::rgb(
        color.r().sqrt(),
        color.g().sqrt(),
        color.b().sqrt(),
    )
}