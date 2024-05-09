use bevy::prelude::{Color, EventWriter};
use bevy_egui::{egui, EguiContexts};
use crate::elements2d::tunnelgon::{LaserAnimationEvent, RingAnimationEvent, RingBasePosAnim, RingBaseValAnim, SetTunnelgonEvent, TunnelgonBaseAnim};
use crate::elements2d::zoomagon::SpawnZoomagonEvent;
use crate::hexagon::HexagonDefinition;

pub fn elements_2d_gui(
    mut contexts: EguiContexts,
    mut spawn_zoomagon_event_writer: EventWriter<SpawnZoomagonEvent>,
    mut set_tunnelgon_event_writer: EventWriter<SetTunnelgonEvent>,
    mut laser_animation_event_writer: EventWriter<LaserAnimationEvent>,
    mut ring_animation_event_writer: EventWriter<RingAnimationEvent>,
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
        if ui.button("Tunnelgon All").clicked() {
            set_tunnelgon_event_writer.send(
                SetTunnelgonEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                        HexagonDefinition::B1,
                        HexagonDefinition::B2,
                        HexagonDefinition::B3,
                    ]
                }
            );
        };
        if ui.button("Tunnelgon Off").clicked() {
            set_tunnelgon_event_writer.send(
                SetTunnelgonEvent {
                    affected_hexagons: vec![
                    ]
                }
            );
        };
        if ui.button("Laser Pulse").clicked() {
            laser_animation_event_writer.send(
                LaserAnimationEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                        HexagonDefinition::B1,
                        HexagonDefinition::B2,
                        HexagonDefinition::B3,
                    ],
                    base_anim: TunnelgonBaseAnim::Pulse,
                    indices: vec![0, 1, 2, 3, 4, 5, 6, 7],
                    values: vec![1.; 8],
                }
            );
        };
        if ui.button("Laser Full").clicked() {
            laser_animation_event_writer.send(
                LaserAnimationEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                        HexagonDefinition::B1,
                        HexagonDefinition::B2,
                        HexagonDefinition::B3,
                    ],
                    base_anim: TunnelgonBaseAnim::SetToVal,
                    indices: vec![0, 1, 2, 3, 4, 5, 6, 7],
                    values: vec![1.; 8],
                }
            );
        };
        if ui.button("Laser Off").clicked() {
            laser_animation_event_writer.send(
                LaserAnimationEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                        HexagonDefinition::B1,
                        HexagonDefinition::B2,
                        HexagonDefinition::B3,
                    ],
                    base_anim: TunnelgonBaseAnim::SetToVal,
                    indices: vec![0, 1, 2, 3, 4, 5, 6, 7],
                    values: vec![0.; 8],
                }
            );
        };
        if ui.button("Rings").clicked() {
            ring_animation_event_writer.send(
                RingAnimationEvent {
                    affected_hexagons: vec![
                        HexagonDefinition::A1,
                        HexagonDefinition::A2,
                        HexagonDefinition::A3,
                        HexagonDefinition::B1,
                        HexagonDefinition::B2,
                        HexagonDefinition::B3,
                    ],
                    base_pos_anim: RingBasePosAnim::SlideLinear,
                    base_val_anim: RingBaseValAnim::Pulse,
                    indices: vec![0, 1, 2, 3, 4,],
                    positions_from: vec![0., 0.1, 0.2, 0.3, 0.4],
                    positions_to: vec![1.1, 1.2, 1.3, 1.4, 1.5],
                    values: vec![1.; 5],
                }
            );
        };
    });
}