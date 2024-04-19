use bevy::prelude::EventWriter;
use bevy_egui::{egui, EguiContexts};
use crate::hexagon::HexagonDefinition;
use crate::physics_hexagon::effectors::center_pull::CenterPullEvent;

pub fn effectors_gui(
    mut contexts: EguiContexts,
    mut center_pull_event_writer: EventWriter<CenterPullEvent>,
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
                }
            );
        };
    });
}