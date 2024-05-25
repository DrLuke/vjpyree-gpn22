use std::collections::VecDeque;
use bevy::ecs::event::ManualEventReader;
use bevy::ecs::system::SystemParam;
use bevy::input::ButtonInput;
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, default, EventReader, Events, EventWriter, KeyCode, Local, OrthographicProjection, Query, Res, ResMut, Resource, Window};
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::window::{PresentMode, WindowRef, WindowResolution};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Ui, WidgetText};
use egui_plot::{Line, Plot, PlotBounds, PlotPoints};
use rand::{Rng, thread_rng};
use crate::beat::BeatEvent;
use crate::beat::bpm_guesser::BpmGuesser;
use crate::elements2d::pedrogon::SetPedrogonEvent;
use crate::elements2d::swirlagon::SetSwirlagonEvent;
use crate::elements2d::tunnelgon::SetTunnelgonEvent;
use crate::hexagon::HexagonDefinition;
use crate::hexagon::HexagonDefinition::Main;
use crate::propagating_render_layers::PropagatingRenderLayers;
use crate::swirl::SwirlAutomation;
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
    beat_reader: Local<'s, ManualEventReader<BeatEvent>>,
    beat_events: ResMut<'w, Events<BeatEvent>>,
    traktor_beat: ResMut<'w, TraktorBeat>,
    keys: Res<'w, ButtonInput<KeyCode>>,
    beat_mute: ResMut<'w, BeatMute>,
    bpm_guesser: Res<'w, BpmGuesser>,
    bpm_data: Local<'s, VecDeque<f32>>,
    plot_bounds: Local<'s, BpmPlotBounds>,
}

#[derive(Default)]
pub struct NextSettings {
    swirl_next_beat: bool,
    swirl_preset: usize,
    gons_next_beat: bool,
    tunnelgon: Vec<HexagonDefinition>,
    swirlgon: Vec<HexagonDefinition>,
    pedrogon: Vec<HexagonDefinition>,
}

pub fn left_panel(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut beat_controls_params: BeatControlsParams,
    mut swirl: ResMut<SwirlAutomation>,
    mut next_settings: Local<NextSettings>,
    mut tunnelgon_reader: Local<ManualEventReader<SetTunnelgonEvent>>,
    mut tunnelgon_events: ResMut<Events<SetTunnelgonEvent>>,
    mut swirlgon_reader: Local<ManualEventReader<SetSwirlagonEvent>>,
    mut swirlgon_events: ResMut<Events<SetSwirlagonEvent>>,
    mut pedrogon_reader: Local<ManualEventReader<SetPedrogonEvent>>,
    mut pedrogon_events: ResMut<Events<SetPedrogonEvent>>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("Left Panel GUI")
        .resizable(false)
        .default_width(300.)
        .show(ctx, |ui| {
            let is_beat = beat_controls_params.beat_reader.read(&beat_controls_params.beat_events).len() > 0;


            ui.heading("App Controls");
            ui.separator();

            // ----------------------------------------------
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
            ui.separator();


            // ----------------------------------------------
            // BPM CONTROLS

            // Gather BPM data
            if beat_controls_params.bpm_data.len() >= 300 { beat_controls_params.bpm_data.pop_front(); }
            beat_controls_params.bpm_data.push_back(beat_controls_params.bpm_guesser.calculate_bpm());


            ui.horizontal(|ui| {
                ui.label("Beat: ");
                if is_beat {
                    ui.label("BEAT");
                }
            });
            if ui.button("Send Beat").clicked() {
                beat_controls_params.beat_events.send(
                    BeatEvent {
                        count: 0,
                        bpm: None,
                    }
                );
            }
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

            // ----------------------------------------------
            // Swirl controls

            ui.separator();
            ui.heading("Swirl");

            ui.horizontal(|ui| {
                ui.checkbox(&mut swirl.fix_pal, "Fix Palette");
                ui.checkbox(&mut swirl.fix_fb_rot, "Fix FB rot");
                ui.checkbox(&mut next_settings.swirl_next_beat, "Set on beat")
            });

            ui.horizontal(|ui| {
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 0, "Green Portal");
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 1, "Blur out");
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 2, "Rainbow out");
            });
            ui.horizontal(|ui| {
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 3, "Fractal 1");
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 4, "Fractal 2");
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 5, "Fractal 3");
            });
            ui.horizontal(|ui| {
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 6, "Blue");
                swirl_preset_button(ui, &mut next_settings.swirl_preset, 7, "Red");
                if ui.add_sized([80., 30.], egui::Button::new("Random")).clicked() {
                    let mut rng = thread_rng();
                    next_settings.swirl_preset = rng.gen_range(0..=7)
                };
            });

            // ----------------------------------------------
            // X-GON SELECT
            // Read changed from external
            ui.separator();
            ui.heading("Gons");
            for ev in tunnelgon_reader.read(&tunnelgon_events) {
                next_settings.tunnelgon = ev.affected_hexagons.clone();
            }
            for ev in swirlgon_reader.read(&swirlgon_events) {
                next_settings.swirlgon = ev.affected_hexagons.clone();
            }
            for ev in pedrogon_reader.read(&pedrogon_events) {
                next_settings.pedrogon = ev.affected_hexagons.clone();
            }

            for hex in vec![HexagonDefinition::A1, HexagonDefinition::A2, HexagonDefinition::A3,
                            HexagonDefinition::B1, HexagonDefinition::B2, HexagonDefinition::B3, HexagonDefinition::Main] {
                ui.horizontal(|ui| {
                    ui.label(format!("{:?}", hex));
                    if ui.add_sized([60., 30.], egui::SelectableLabel::new(next_settings.tunnelgon.contains(&hex), "Tunnelgon"))
                        .clicked() {
                        insert_hex_to_vec(&mut next_settings.tunnelgon, &hex);
                        remove_hex_from_vec(&mut next_settings.swirlgon, &hex);
                        remove_hex_from_vec(&mut next_settings.pedrogon, &hex);
                    };
                    if ui.add_sized([60., 30.], egui::SelectableLabel::new(next_settings.swirlgon.contains(&hex), "Swirlgon"))
                        .clicked() {
                        insert_hex_to_vec(&mut next_settings.swirlgon, &hex);
                        remove_hex_from_vec(&mut next_settings.tunnelgon, &hex);
                        remove_hex_from_vec(&mut next_settings.pedrogon, &hex);
                    };
                    if ui.add_sized([60., 30.], egui::SelectableLabel::new(next_settings.pedrogon.contains(&hex), "Pedrogon"))
                        .clicked() {
                        insert_hex_to_vec(&mut next_settings.pedrogon, &hex);
                        remove_hex_from_vec(&mut next_settings.swirlgon, &hex);
                        remove_hex_from_vec(&mut next_settings.tunnelgon, &hex);
                    };
                    if hex == HexagonDefinition::Main {
                        if ui.button("Off").clicked() {
                            remove_hex_from_vec(&mut next_settings.pedrogon, &hex);
                            remove_hex_from_vec(&mut next_settings.swirlgon, &hex);
                            remove_hex_from_vec(&mut next_settings.tunnelgon, &hex);
                        }
                    }
                });
            }
            if ui.button("Set").clicked() {
                next_settings.gons_next_beat = true;
            }

            if is_beat {
                if next_settings.swirl_next_beat {
                    swirl.preset = next_settings.swirl_preset;
                }

                if next_settings.gons_next_beat {
                    tunnelgon_events.send(SetTunnelgonEvent {
                        affected_hexagons: next_settings.tunnelgon.clone()
                    });
                    swirlgon_events.send(SetSwirlagonEvent {
                        affected_hexagons: next_settings.swirlgon.clone()
                    });
                    pedrogon_events.send(SetPedrogonEvent {
                        affected_hexagons: next_settings.pedrogon.clone()
                    });
                    next_settings.gons_next_beat = false;
                }
            }

            // ----------------------------------------------

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}

fn swirl_preset_button(ui: &mut Ui, preset: &mut usize, index: usize, text: impl Into<WidgetText>) {
    if ui.add_sized([80., 30.], egui::SelectableLabel::new(*preset == index, text))
        .clicked() {
        *preset = index;
    };
}

fn insert_hex_to_vec(vec: &mut Vec<HexagonDefinition>, hex: &HexagonDefinition) {
    if !vec.contains(hex) {
        vec.push(*hex);
    }
}

fn remove_hex_from_vec(vec: &mut Vec<HexagonDefinition>, hex: &HexagonDefinition) {
    *vec = vec.iter().filter(|h| *hex != **h).cloned().collect();
}