/// Counts the time between beats and estimates BPM

use std::collections::VecDeque;
use bevy::prelude::{EventReader, Real, Res, ResMut, Resource, Time};
use crate::beat::BeatEvent;

#[derive(Resource)]
pub struct BpmGuesser {
    // Times at which beat event arrived
    pub samples: VecDeque<f32>,
    pub prev_time: f32,
}

impl Default for BpmGuesser {
    fn default() -> Self {
        Self {
            samples: VecDeque::with_capacity(64),
            prev_time: 0.,
        }
    }
}

impl BpmGuesser {
    pub fn calculate_bpm(&self) -> f32 {
        60./ (self.samples.iter().fold(0., |a, v| a+v) / self.samples.len() as f32)
    }
}

pub fn bpm_guesser_system(
    mut bpm_guesser: ResMut<BpmGuesser>,
    mut beat_event: EventReader<BeatEvent>,
    time: Res<Time<Real>>,
) {
    for ev in beat_event.read() {
        bpm_guesser.samples.pop_front();
        let t = time.elapsed_seconds();
        let s = t - bpm_guesser.prev_time.clone();
        bpm_guesser.samples.push_back(s);
        bpm_guesser.prev_time = t;
    }
}