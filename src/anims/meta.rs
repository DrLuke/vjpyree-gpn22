use std::future::join;
/// Meta animations that trigger oneshots
use bevy::prelude::{Commands, error, EventReader, EventWriter, ResMut, Resource};
use bevy_defer::{AsyncCommandsExtension, world};
use crate::beat::BeatEvent;
use crate::elements2d::tunnelgon::LaserAnimationEvent;
use crate::elements2d::tunnelgon::TunnelgonBaseAnim::Pulse;
use crate::hexagon::HexagonDefinition;


#[derive(Resource, Default)]
pub struct TunnelgonLaserCycleMetaAnim {
    enabled: bool,
    counter: usize,
}

pub fn tunnelgon_laser_cycle_meta_anim(
    mut params: ResMut<TunnelgonLaserCycleMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut event_writer: EventWriter<LaserAnimationEvent>,
) {
    if !params.enabled { return; }
    for _ in beat_reader.read() {
        let hex = match params.counter {
            1 => vec![HexagonDefinition::A2, HexagonDefinition::B2],
            2 => vec![HexagonDefinition::A3, HexagonDefinition::B3],
            0 | _ => vec![HexagonDefinition::A1, HexagonDefinition::B1],
        };
        event_writer.send(LaserAnimationEvent {
            affected_hexagons: hex,
            base_anim: Pulse,
            indices: vec![0, 1, 2, 3, 4, 5],
            values: vec![1., 1., 1., 1., 1., 1.],
        });
        params.counter = (params.counter + 1) % 3;
    }
}

#[derive(Resource, Default)]
pub struct TunnelgonLaserFigureEightMetaAnim {
    enabled: bool,
}

pub fn tunnelgon_laser_figure_eight_meta_anim(
    mut params: ResMut<TunnelgonLaserFigureEightMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut commands: Commands,
) {
    //if !params.enabled { return; }
    for _ in beat_reader.read() {
        commands.spawn_task(|| async move {
            let params = world().resource::<TunnelgonLaserFigureEightMetaAnim>();

            let indices = vec![
                // First
                (2, 1),
                (3, 0),
                (4, 5),
                (5, 4),
                (0, 3),
                (1, 2),
                // Second
                (4, 5),
                (3, 0),
                (2, 1),
                (1, 2),
                (0, 3),
                (5, 4),
            ];

            for (i, (left_index, right_index)) in indices.iter().enumerate() {
                let (hex_l, hex_r) = if i < 6 {
                    (HexagonDefinition::A1, HexagonDefinition::B1)
                } else {
                    (HexagonDefinition::A3, HexagonDefinition::B3)
                };
                let left_event = world().send_event::<LaserAnimationEvent>(
                    LaserAnimationEvent {
                        affected_hexagons: vec![hex_l],
                        base_anim: Pulse,
                        indices: vec![*left_index],
                        values: vec![1.],
                    });
                let right_event = world().send_event::<LaserAnimationEvent>(
                    LaserAnimationEvent {
                        affected_hexagons: vec![hex_r],
                        base_anim: Pulse,
                        indices: vec![*right_index],
                        values: vec![1.],
                    });
                let _ = join!(left_event, right_event, world().sleep_frames(2)).await;
            }

            Ok(())
        });
    }
}