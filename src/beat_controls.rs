use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::beat::BeatEvent;
use crate::traktor_beat::TraktorBeat;


pub struct BeatControls;

impl Plugin for BeatControls {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, beat_ui)
            .insert_resource(BeatMute::default())
        ;
    }
}

#[derive(Resource, Default)]
pub struct BeatMute{
    pub mute: bool
}

pub fn beat_ui(
    mut contexts: EguiContexts,
    mut beat_reader: EventReader<BeatEvent>,
    mut traktor_beat: ResMut<TraktorBeat>,
    keys: Res<ButtonInput<KeyCode>>,
    mut beat_mute: ResMut<BeatMute>,
) {
    egui::Window::new("Beat").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Beat: ");
            if beat_reader.read().len() > 0 {
                ui.label("BEAT");
            }
        });
        ui.label(format!("{}", traktor_beat.count));
        ui.add(egui::ProgressBar::new((traktor_beat.count as f32 / 24.))
            .show_percentage());
        ui.add(egui::ProgressBar::new((traktor_beat.last_volume as f32 / 128.))
            .show_percentage());

        ui.horizontal(|ui| {
            if ui.button("Decr").clicked() { traktor_beat.count -= 1; }
            if ui.button("Incr").clicked() { traktor_beat.count += 1; }
        });

        ui.separator();

        ui.checkbox(&mut beat_mute.mute, "Beat Mute");
        if keys.just_pressed(KeyCode::Space) {
            beat_mute.mute = true;
        }
        if keys.just_released(KeyCode::Space) {
            beat_mute.mute = false;
        }


    });
}