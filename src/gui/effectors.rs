use std::f32::consts::PI;
use bevy::prelude::{EventWriter, Local, ResMut};
use bevy::utils::default;
use bevy_egui::{egui, EguiContexts};
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::effectors::center_pull::CenterPullEvent;
use crate::physics_hexagon::effectors::center_push::CenterPushEvent;
use crate::physics_hexagon::effectors::dir_push::DirPushEvent;
use crate::physics_hexagon::effectors::{EyesMode, PhysHexSettings};

pub fn effectors_gui(
    mut contexts: EguiContexts,
    mut center_pull_event_writer: EventWriter<CenterPullEvent>,
    mut center_push_event_writer: EventWriter<CenterPushEvent>,
    mut dir_push_event_writer: EventWriter<DirPushEvent>,
    mut settings: ResMut<PhysHexSettings>,
    mut dir_push_event: Local<DirPushEvent>,
) {
    egui::Window::new("Effectors").show(contexts.ctx_mut(), |ui| {
        if ui.button("Center Pull").clicked() {
            center_pull_event_writer.send(
                CenterPullEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::Main,
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                    ],
                    ..default()
                }
            );
        };
        if ui.button("Center Push").clicked() {
            center_push_event_writer.send(
                CenterPushEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::Main,
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                    ],
                }
            );
        };
        ui.horizontal(|ui| {
            if ui.button("Dir push").clicked() {
                dir_push_event_writer.send(
                    dir_push_event.clone()
                );
            };
            ui.add(egui::DragValue::new(&mut dir_push_event.dir).speed(0.01).clamp_range(0.0..=PI * 2.));
        });

        egui::ComboBox::from_label("Eyes mode")
            .selected_text(format!("{:?}", settings.eyes_mode))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut settings.eyes_mode, EyesMode::None, "None");
                ui.selectable_value(&mut settings.eyes_mode, EyesMode::Crazy, "Crazy");
                ui.selectable_value(&mut settings.eyes_mode, EyesMode::Stare, "Stare");
            })
    });
}