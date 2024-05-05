use bevy::prelude::{Color, EventWriter};
use bevy_egui::{egui, EguiContexts};
use crate::elements2d::zoomagon::SpawnZoomagonEvent;
use crate::hexagon::HexagonDefinition;

pub fn elements_2d_gui(
    mut contexts: EguiContexts,
    mut spawn_zoomagon_event_writer: EventWriter<SpawnZoomagonEvent>,
) {
    egui::Window::new("Elements 2D").show(contexts.ctx_mut(), |ui| {
        if ui.button("Zoomagon").clicked() {
            spawn_zoomagon_event_writer.send(
                SpawnZoomagonEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                        HexagonDefinition::B1,
                        HexagonDefinition::B2,
                        HexagonDefinition::B3,
                    ],
                    color: Color::RED
                }
            );
        };
    });
}