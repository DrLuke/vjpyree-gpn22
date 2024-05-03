use bevy::app::{App, Plugin};
use bevy::prelude::{Camera, Camera2d, Camera2dBundle, ClearColorConfig, Color, Commands, default, OrthographicProjection, Res, Update, Window};
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::window::{PresentMode, WindowRef, WindowResolution};
use bevy_egui::{egui, EguiContexts};
use crate::hexagon::render::HexagonRenderTarget;
use crate::propagating_render_layers::PropagatingRenderLayers;

pub struct OutputWindowPlugin;

impl Plugin for OutputWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ui_system);
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut commands: Commands,
    rt: Res<HexagonRenderTarget>,
) {
    egui::Window::new("Projection Map").show(contexts.ctx_mut(), |ui| {
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
    });
}