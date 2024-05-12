//! Components and systems related to beat signals

use bevy::prelude::*;

mod osc_receiver;
mod plugin;

pub use osc_receiver::{OscBeatReceiver, osc_beat_receiver_system};
pub use plugin::OscBeatReceiverPlugin;

/// Resource that counts how many beats have been received
#[derive(Resource, Default)]
pub struct BeatCounter {
    // Counter is increased on every beat
    pub count: u64,
}

impl BeatCounter {
    pub fn get_count(&self) -> u64 { self.count }
}

/// Event that is emitted when a beat occurs
#[derive(Event)]
pub struct BeatEvent {
    /// Value from BeatCounter
    pub count: u64,
    /// Optional BPM value if it is sent
    pub bpm: Option<f32>
}