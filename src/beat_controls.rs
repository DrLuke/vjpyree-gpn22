use std::collections::VecDeque;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_plot::{Line, Plot, PlotPoints, VLine};
use crate::beat::BeatEvent;
use crate::beat::bpm_guesser::BpmGuesser;
use crate::elements2d::tunnelgon::{LaserAnimationEvent, TunnelgonBaseAnim};
use crate::hexagon::HexagonDefinition;
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
    bpm_guesser: Res<BpmGuesser>,
    mut bpm_data: Local<VecDeque<f32>>
) {
    // Gather BPM data
    if bpm_data.len() >= 300 { bpm_data.pop_front(); }
    bpm_data.push_back(bpm_guesser.calculate_bpm());

    egui::Window::new("Beat").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Beat: ");
            if beat_reader.read().len() > 0 {
                ui.label("BEAT");
            }
        });

        let values: Vec<f64> = bpm_data.iter().map(|a| a.clone() as f64).collect();
        let line_points: PlotPoints = (0..bpm_data.len())
            .map(|i| {
                let x = egui::remap(i as f64, 0f64..=bpm_data.len() as f64, (-(bpm_data.len() as f64)+1.)..=0.);
                [x, *values.get(i).unwrap_or(&0f64)]
            })
            .collect();
        let line = Line::new(line_points);
        Plot::new("example_plot")
            .height(64.0)
            .show_axes(false)
            .data_aspect(1.0)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            })
            .response;

        ui.separator();

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