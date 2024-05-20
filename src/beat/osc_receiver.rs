///! Receive beat signals via OSC

use bevy::prelude::*;
use bevy_rosc::{SingleAddressOscMethod};
use rosc::OscType;
use crate::beat::{BeatCounter, BeatEvent};
use crate::gui::left_panel::BeatMute;

#[derive(Component)]
pub struct OscBeatReceiver;

/// Whenever a message is received at the provided beat address, increment the beat counter and send an event
pub fn osc_beat_receiver_system(
    mut beat_counter: ResMut<BeatCounter>,
    mut beat_writer: EventWriter<BeatEvent>,
    mut query: Query<&mut SingleAddressOscMethod, (With<OscBeatReceiver>, Changed<SingleAddressOscMethod>)>,
    beat_mute: Res<BeatMute>,
) {
    for mut osc_method in query.iter_mut() {
        while let Some(message) = osc_method.get_message() {
            beat_counter.count += 1;
            // Check if BPM info was in the message
            let bpm = match message.args.first() {
                Some(OscType::Float(bpm)) => Some(*bpm),
                _ => None,
            };
            if !beat_mute.mute {
                beat_writer.send(BeatEvent { count: beat_counter.count, bpm });
            }
        }
    }
}

