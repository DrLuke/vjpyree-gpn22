use bevy::app::App;
use bevy::hierarchy::Children;
use bevy::prelude::{Commands, Entity, Plugin, Query, Startup, Update, With};
use strum::IntoEnumIterator;
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::lights::led_tube::{LedTube, LedTubeLed, spawn_tube, TubeIndex};
use crate::physics_hexagon::lights::physical_lights::spawn_physical_led_tube;
use crate::physics_hexagon::PhysicsHexagon;

pub mod led_tube;
pub mod physical_lights;

pub fn spawn_led_tubes(
    mut commands: Commands
) {
    for index in TubeIndex::iter() {
        spawn_tube(index, &mut commands)
    }
}

pub fn spawn_physical_lights(
    mut commands: Commands,
    physical_hexagon_query: Query<(Entity, &Children, &PhysicsHexagon)>,
    hexagon_lights_query: Query<Entity, With<PhysicsHexagon>>,
    led_tube_query: Query<(&Children, &LedTube), With<LedTube>>,
    led_tube_led_query: Query<(Entity, &LedTubeLed)>,
) {
    let config = vec![
        // Main Hexagon
        (HexagonDefinition::Main, TubeIndex::Six),
        (HexagonDefinition::Main, TubeIndex::Seven),
        (HexagonDefinition::Main, TubeIndex::Eight),
        (HexagonDefinition::Main, TubeIndex::Nine),
        (HexagonDefinition::Main, TubeIndex::Ten),
        (HexagonDefinition::Main, TubeIndex::Eleven),
        (HexagonDefinition::Main, TubeIndex::Twelve),
        (HexagonDefinition::Main, TubeIndex::Thirteen),
        (HexagonDefinition::Main, TubeIndex::Fourteen),
        (HexagonDefinition::Main, TubeIndex::Fifteen),
        (HexagonDefinition::Main, TubeIndex::Sixteen),
        (HexagonDefinition::Main, TubeIndex::Seventeen),
    ];

    for (hd, ti) in config {
        spawn_physical_led_tube(
            ti, hd,
            &mut commands,
            &physical_hexagon_query,
            &hexagon_lights_query,
            &led_tube_query,
            &led_tube_led_query
        );
    };
}