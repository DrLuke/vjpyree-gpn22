use std::cmp::PartialEq;
use std::f32::consts::PI;
use bevy::prelude::*;
use crate::beat::BeatEvent;
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::effectors::center_pull::CenterPullEvent;
use crate::physics_hexagon::effectors::center_push::CenterPushEvent;
use crate::physics_hexagon::effectors::dir_push::DirPushEvent;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum PhysAnimMode {
    #[default]
    None,
    Push,
    Pull,
    PushPull,
    ContPull,
    Sides,
}

#[derive(Resource, Default)]
pub struct PhysMetaAnim {
    pub anim_mode: PhysAnimMode,
    push_pull_counter: usize,
    dir_push_counter: usize,
}

pub fn push_or_pull_meta_anim(
    mut push_writer: EventWriter<CenterPushEvent>,
    mut pull_writer: EventWriter<CenterPullEvent>,
    mut beat_reader: EventReader<BeatEvent>,
    settings: Res<PhysMetaAnim>,
) {
    if settings.anim_mode != PhysAnimMode::Push && settings.anim_mode != PhysAnimMode::Pull && settings.anim_mode != PhysAnimMode::ContPull { return; }

    for _ in beat_reader.read() {
        if settings.anim_mode == PhysAnimMode::Push {
            push_writer.send(CenterPushEvent {
                affected_hexagons: vec![HexagonDefinition::Main]
            });
        }
        if settings.anim_mode == PhysAnimMode::Pull {
            pull_writer.send(CenterPullEvent {
                affected_hexagons: vec![HexagonDefinition::Main],
                ..default()
            });
        }
    }

    if settings.anim_mode == PhysAnimMode::ContPull {
        pull_writer.send(CenterPullEvent {
            affected_hexagons: vec![HexagonDefinition::Main],
            strength: 80000.,
        });
    }
}

pub fn push_pull_meta_anim(
    mut push_writer: EventWriter<CenterPushEvent>,
    mut pull_writer: EventWriter<CenterPullEvent>,
    mut beat_reader: EventReader<BeatEvent>,
    mut settings: ResMut<PhysMetaAnim>,
) {
    if settings.anim_mode != PhysAnimMode::PushPull { return; }

    for _ in beat_reader.read() {
        if settings.push_pull_counter == 0 {
            push_writer.send(CenterPushEvent {
                affected_hexagons: vec![HexagonDefinition::Main]
            });
        }
        if settings.push_pull_counter == 1 {
            pull_writer.send(CenterPullEvent {
                affected_hexagons: vec![HexagonDefinition::Main],
                ..default()
            });
        }
        settings.push_pull_counter = (settings.push_pull_counter + 1) % 2
    }
}

pub fn sides_meta_anim(
    mut push_writer: EventWriter<DirPushEvent>,
    mut beat_reader: EventReader<BeatEvent>,
    mut settings: ResMut<PhysMetaAnim>,
) {
    if settings.anim_mode != PhysAnimMode::Sides { return; }

    let dirs = vec![
        0.*PI/3.,
        1.*PI/3.,
        2.*PI/3.,
        3.*PI/3.,
        4.*PI/3.,
        5.*PI/3.,
    ];

    for _ in beat_reader.read() {
        let dir = dirs[settings.dir_push_counter];

        push_writer.send(DirPushEvent {
            dir
        });

        settings.dir_push_counter = (settings.dir_push_counter + 1) % 6;
    }
}