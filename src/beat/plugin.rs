use bevy::prelude::*;
use crate::beat::osc_receiver::osc_beat_receiver_system;
use crate::beat::{BeatCounter, BeatEvent, OscBeatReceiver};
use bevy_rosc::{SingleAddressOscMethod, method_dispatcher_system};
use crate::beat::bpm_guesser::{bpm_guesser_system, BpmGuesser};

pub struct OscBeatReceiverPlugin {
    /// Address at which the osc beat signal comes in
    pub address: String,
}

impl Default for OscBeatReceiverPlugin {
    fn default() -> Self {
        Self {
            address: "/beat".to_owned()
        }
    }
}

impl Plugin for OscBeatReceiverPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BeatCounter::default())
            .add_event::<BeatEvent>()
            .add_systems(PreUpdate, osc_beat_receiver_system.after(method_dispatcher_system::<SingleAddressOscMethod>))
            .insert_resource(BpmGuesser::default())
            .add_systems(PreUpdate, bpm_guesser_system.after(osc_beat_receiver_system))
        ;
        app.world.spawn((
            OscBeatReceiver {},
            SingleAddressOscMethod::new(self.address.clone()).unwrap()
        ));
    }
}