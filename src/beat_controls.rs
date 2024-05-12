use std::collections::VecDeque;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints, VLine};
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
pub struct BeatMute {
    pub mute: bool,
}

struct BpmPlotBounds(pub f32, pub f32);

impl Default for BpmPlotBounds {
    fn default() -> Self {
        Self(140., 300.)
    }
}

pub fn beat_ui(
    mut contexts: EguiContexts,
    mut beat_reader: EventReader<BeatEvent>,
    mut traktor_beat: ResMut<TraktorBeat>,
    keys: Res<ButtonInput<KeyCode>>,
    mut beat_mute: ResMut<BeatMute>,
    bpm_guesser: Res<BpmGuesser>,
    mut bpm_data: Local<VecDeque<f32>>,
    mut plot_bounds: Local<BpmPlotBounds>
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
        ui.horizontal(|ui| {
            ui.label("BPM: ");
            ui.label(format!("{}", bpm_data.back().unwrap_or(&0.)));
        });
        ui.horizontal(|ui| {
            ui.label("Mid:");
            ui.add(egui::DragValue::new(&mut plot_bounds.0).speed(1.0));
            ui.label("Width:");
            ui.add(egui::DragValue::new(&mut plot_bounds.1).speed(1.0));
        });

        let values: Vec<f64> = bpm_data.iter().map(|a| a.clone() as f64).collect();
        let line_points: PlotPoints = (0..bpm_data.len())
            .map(|i| {
                let x = egui::remap(i as f64, 0f64..=bpm_data.len() as f64, (-(bpm_data.len() as f64) + 1.)..=0.);
                [x, *values.get(i).unwrap_or(&0f64)]
            })
            .collect();
        let line = Line::new(line_points);
        Plot::new("BPM")
            .height(64.0)
            .show_axes(false)
            .show(ui, |plot_ui| {
                plot_ui.set_plot_bounds(PlotBounds::from_min_max([-(bpm_data.len() as f64) + 1., (plot_bounds.0 - plot_bounds.1/2.) as f64], [0., (plot_bounds.0 + plot_bounds.1/2.) as f64]));
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