use std::future::join;
/// Meta animations that trigger oneshots
use bevy::prelude::{Commands, error, EventReader, EventWriter, ResMut, Resource};
use bevy_defer::{AsyncCommandsExtension, world};
use crate::beat::BeatEvent;
use crate::elements2d::tunnelgon::{LaserAnimationEvent, RingAnimationEvent, RingBasePosAnim, RingBaseValAnim};
use crate::elements2d::tunnelgon::TunnelgonBaseAnim::Pulse;
use crate::hexagon::HexagonDefinition;


#[derive(Resource, Default)]
pub struct TunnelgonLaserCycleMetaAnim {
    pub enabled: bool,
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
    pub enabled: bool,
}

pub fn tunnelgon_laser_figure_eight_meta_anim(
    mut params: ResMut<TunnelgonLaserFigureEightMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut commands: Commands,
) {
    if !params.enabled { return; }
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

#[derive(Resource, Default)]
pub struct TunnelgonLaserRoundTheClockMetaAnim {
    pub enabled: bool,
    counter: usize,
}

pub fn tunnelgon_laser_round_the_clock_meta_anim(
    mut params: ResMut<TunnelgonLaserRoundTheClockMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut laser_writer: EventWriter<LaserAnimationEvent>,
) {
    if !params.enabled { return; }
    for ev in beat_reader.read() {
        let left_index = params.counter as isize;
        let right_index = 12 - (params.counter as isize);
        laser_writer.send(
            LaserAnimationEvent {
                affected_hexagons: vec![HexagonDefinition::A1, HexagonDefinition::A2, HexagonDefinition::A3],
                base_anim: Pulse,
                indices: vec![(left_index % 6) as usize, ((left_index + 3) % 6) as usize],
                values: vec![1., 1.],
            });
        laser_writer.send(
            LaserAnimationEvent {
                affected_hexagons: vec![HexagonDefinition::B1, HexagonDefinition::B2, HexagonDefinition::B3],
                base_anim: Pulse,
                indices: vec![(right_index % 6) as usize, ((right_index - 3) % 6) as usize],
                values: vec![1., 1.],
            });
        println!("Counter: {}, left: {:?}, right: {:?}", params.counter, vec![(left_index % 6) as usize, ((left_index + 3) % 6) as usize], vec![(right_index % 6) as usize, ((right_index - 3) % 6) as usize]);
        params.counter = (params.counter + 1) % 6;
    }
}

#[derive(Resource, Default)]
pub struct TunnelgonLaserSweepMetaAnim {
    pub enabled: bool,
    counter: usize,
}

pub fn tunnelgon_laser_sweep_anim(
    mut params: ResMut<TunnelgonLaserSweepMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut laser_writer: EventWriter<LaserAnimationEvent>,
) {
    if !params.enabled { return; }
    for _ in beat_reader.read() {
        let mut indices = vec![
            (4, 5),
            (3, 0),
            (2, 1),
            (3, 0),
        ];

        let ind = indices[params.counter];
        laser_writer.send(LaserAnimationEvent {
            affected_hexagons: vec![HexagonDefinition::A1, HexagonDefinition::A2, HexagonDefinition::A3, HexagonDefinition::B1, HexagonDefinition::B2, HexagonDefinition::B3, HexagonDefinition::Main],
            base_anim: Pulse,
            indices: vec![ind.0, ind.1],
            values: vec![1., 1.],
        });

        params.counter = (params.counter + 1) % 4;
    }
}

#[derive(Resource, Default)]
pub struct TunnelgonRingsFTBMetaAnim {
    pub enabled: bool,
    counter: usize,
}

pub fn tunnelgon_rings_ftb_meta_anim(
    mut params: ResMut<TunnelgonRingsFTBMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut ring_writer: EventWriter<RingAnimationEvent>,
) {
    if !params.enabled { return; }
    for _ in beat_reader.read() {
        ring_writer.send(RingAnimationEvent {
            affected_hexagons: vec![HexagonDefinition::A1, HexagonDefinition::A2, HexagonDefinition::A3, HexagonDefinition::B1, HexagonDefinition::B2, HexagonDefinition::B3, HexagonDefinition::Main],
            base_pos_anim: RingBasePosAnim::SlideLinear,
            base_val_anim: RingBaseValAnim::Pulse,
            indices: vec![params.counter],
            values: vec![1.],
            positions_from: vec![0.],
            positions_to: vec![1.],
        });
    }
    params.counter = (params.counter + 1) % 2;
}

#[derive(Resource, Default)]
pub struct TunnelgonRingsBTFMetaAnim {
    pub enabled: bool,
    counter: usize,
}

pub fn tunnelgon_rings_btf_meta_anim(
    mut params: ResMut<TunnelgonRingsBTFMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut ring_writer: EventWriter<RingAnimationEvent>,
) {
    if !params.enabled { return; }
    for _ in beat_reader.read() {
        ring_writer.send(RingAnimationEvent {
            affected_hexagons: vec![HexagonDefinition::A1, HexagonDefinition::A2, HexagonDefinition::A3, HexagonDefinition::B1, HexagonDefinition::B2, HexagonDefinition::B3, HexagonDefinition::Main],
            base_pos_anim: RingBasePosAnim::SlideLinear,
            base_val_anim: RingBaseValAnim::Pulse,
            indices: vec![params.counter + 2],
            values: vec![1.],
            positions_from: vec![0.5],
            positions_to: vec![-0.5],
        });
    }
    params.counter = (params.counter + 1) % 2;
}

#[derive(Resource, Default)]
pub struct TunnelgonRingsTrainMetaAnim {
    pub enabled: bool,
}

pub fn tunnelgon_ring_train_meta_anim(
    mut params: ResMut<TunnelgonRingsTrainMetaAnim>,
    mut beat_reader: EventReader<BeatEvent>,
    mut ring_writer: EventWriter<RingAnimationEvent>,
) {
    if !params.enabled { return; }
    for _ in beat_reader.read() {
        ring_writer.send(RingAnimationEvent {
            affected_hexagons: vec![
                HexagonDefinition::A1,
                HexagonDefinition::A2,
                HexagonDefinition::A3,
                HexagonDefinition::B1,
                HexagonDefinition::B2,
                HexagonDefinition::B3,
                HexagonDefinition::Main,
            ],
            base_pos_anim: RingBasePosAnim::SlideLinear,
            base_val_anim: RingBaseValAnim::Pulse,
            indices: vec![4, 5, 6, 7],
            positions_from: vec![0., 0.1, 0.2, 0.3],
            positions_to: vec![1.1, 1.2, 1.3, 1.4],
            values: vec![1.; 4],
        });
    }
}