use std::collections::VecDeque;
use bevy::ecs::system::SystemParam;
use bevy::input::ButtonInput;
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, default, EventReader, KeyCode, Local, OrthographicProjection, Query, Res, ResMut, Resource, Window};
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::window::{PresentMode, WindowRef, WindowResolution};
use bevy_egui::{egui, EguiContexts};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints};
use crate::beat::BeatEvent;
use crate::beat::bpm_guesser::BpmGuesser;
use crate::propagating_render_layers::PropagatingRenderLayers;
use crate::traktor_beat::TraktorBeat;


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

#[derive(SystemParam)]
pub struct BeatControlsParams<'w, 's> {
    beat_reader: EventReader<'w, 's, BeatEvent>,
    traktor_beat: ResMut<'w, TraktorBeat>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    beat_mute: ResMut<'w, BeatMute>,
    bpm_guesser: Res<'w, BpmGuesser>,
    bpm_data: Local<'s, VecDeque<f32>>,
    plot_bounds: Local<'s, BpmPlotBounds>,
}

pub fn left_panel(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut beat_controls_params: BeatControlsParams,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("Left Panel GUI")
        .resizable(false)
        .default_width(300.)
        .show(ctx, |ui| {


            // SPAWN WINDOW
            if ui.button("Spawn Window").clicked() {
                let second_window = commands
                    .spawn(Window {
                        title: "VJ Pyree output".to_owned(),
                        resolution: WindowResolution::new(1920.0, 1080.0)
                            .with_scale_factor_override(1.0),
                        present_mode: PresentMode::AutoVsync,
                        ..Default::default()
                    })
                    .id();

                commands.spawn((
                    Camera2dBundle {
                        projection: OrthographicProjection {
                            far: 1000.,
                            near: -1000.,
                            scaling_mode: ScalingMode::Fixed { width: 1., height: 1. },
                            ..default()
                        },
                        camera: Camera {
                            order: 9999,
                            target: RenderTarget::Window(WindowRef::Entity(second_window)),
                            clear_color: Color::rgba(0., 0., 0., 0.).into(),
                            ..default()
                        },
                        ..default()
                    },
                    PropagatingRenderLayers { render_layers: RenderLayers::layer(31) }
                ));
            }

            // BPM CONTROLS
            // Gather BPM data
            if beat_controls_params.bpm_data.len() >= 300 { beat_controls_params.bpm_data.pop_front(); }
            beat_controls_params.bpm_data.push_back(beat_controls_params.bpm_guesser.calculate_bpm());


            ui.horizontal(|ui| {
                ui.label("Beat: ");
                if beat_controls_params.beat_reader.read().len() > 0 {
                    ui.label("BEAT");
                }
            });
            ui.horizontal(|ui| {
                ui.label("BPM: ");
                ui.label(format!("{}", beat_controls_params.bpm_data.back().unwrap_or(&0.)));
            });
            ui.horizontal(|ui| {
                ui.label("Mid:");
                ui.add(egui::DragValue::new(&mut beat_controls_params.plot_bounds.0).speed(1.0));
                ui.label("Width:");
                ui.add(egui::DragValue::new(&mut beat_controls_params.plot_bounds.1).speed(1.0));
            });

            let values: Vec<f64> = beat_controls_params.bpm_data.iter().map(|a| a.clone() as f64).collect();
            let line_points: PlotPoints = (0..beat_controls_params.bpm_data.len())
                .map(|i| {
                    let x = egui::remap(i as f64, 0f64..=beat_controls_params.bpm_data.len() as f64, (-(beat_controls_params.bpm_data.len() as f64) + 1.)..=0.);
                    [x, *values.get(i).unwrap_or(&0f64)]
                })
                .collect();
            let line = Line::new(line_points);
            Plot::new("BPM")
                .height(64.0)
                .show_axes(false)
                .show(ui, |plot_ui| {
                    plot_ui.set_plot_bounds(PlotBounds::from_min_max([-(beat_controls_params.bpm_data.len() as f64) + 1., (beat_controls_params.plot_bounds.0 - beat_controls_params.plot_bounds.1 / 2.) as f64], [0., (beat_controls_params.plot_bounds.0 + beat_controls_params.plot_bounds.1 / 2.) as f64]));
                    plot_ui.line(line);
                })
                .response;

            ui.separator();

            ui.label(format!("{}", beat_controls_params.traktor_beat.count));
            ui.add(egui::ProgressBar::new((beat_controls_params.traktor_beat.count as f32 / 24.))
                .show_percentage());
            ui.add(egui::ProgressBar::new((beat_controls_params.traktor_beat.last_volume as f32 / 128.))
                .show_percentage());

            ui.horizontal(|ui| {
                if ui.button("Decr").clicked() { beat_controls_params.traktor_beat.count -= 1; }
                if ui.button("Incr").clicked() { beat_controls_params.traktor_beat.count += 1; }
            });

            ui.separator();

            ui.checkbox(&mut beat_controls_params.beat_mute.mute, "Beat Mute");
            if beat_controls_params.keys.just_pressed(KeyCode::Space) {
                beat_controls_params.beat_mute.mute = true;
            }
            if beat_controls_params.keys.just_released(KeyCode::Space) {
                beat_controls_params.beat_mute.mute = false;
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}