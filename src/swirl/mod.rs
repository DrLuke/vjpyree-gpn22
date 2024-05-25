pub mod render_target;
pub mod swirl_material;

use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, Mesh, OrthographicProjection, Rectangle, RegularPolygon, Res, ResMut, Transform};
use bevy::render::camera::{RenderTarget, ScalingMode};
use bevy::render::view::RenderLayers;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2d, Mesh2dHandle};
use bevy::utils::default;
use bevy_egui::{egui, EguiContexts};
use rand::Rng;
use crate::beat::BeatEvent;
use crate::propagating_render_layers::PropagatingRenderLayers;
use crate::swirl::render_target::SwirlRenderTarget;
use crate::swirl::swirl_material::{SwirlMaterial, SwirlParams};

pub struct SwirlPlugin;

impl Plugin for SwirlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SwirlRenderTarget>();
        app.add_plugins(Material2dPlugin::<SwirlMaterial>::default());
        app.add_systems(Startup, setup_swirl);
        app.add_event::<UpdateSwirlParams>();
        app.add_systems(Update, (update_swirl_params_event, swirl_beat /*, swirl_gui */));
        app.init_resource::<SwirlAutomation>();
    }
}

#[derive(Component)]
pub struct SwirlRenderer;

fn setup_swirl(
    mut commands: Commands,
    mut materials: ResMut<Assets<SwirlMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    rt: Res<SwirlRenderTarget>,
) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                far: 1000.,
                near: -1000.,
                scaling_mode: ScalingMode::Fixed { width: 1., height: 1. },
                ..default()
            },
            camera: Camera {
                order: -1000,
                target: RenderTarget::Image(rt.render_target.clone()),
                clear_color: Color::rgba(0., 0., 0., 0.).into(),
                hdr: true,
                ..default()
            },
            //tonemapping: TonyMcMapface,
            ..default()
        },
        //BloomSettings::NATURAL,
        PropagatingRenderLayers { render_layers: RenderLayers::layer(4) }
    ));

    let mesh = Mesh2dHandle(meshes.add(
        Rectangle::new(1., 1.)
    ));


    commands.spawn((
        MaterialMesh2dBundle {
            mesh,
            material: materials.add(SwirlMaterial {
                prev: rt.render_target.clone(),
                params: SwirlParams::default(),
            }),
            ..default()
        },
        PropagatingRenderLayers { render_layers: RenderLayers::layer(4) },
        SwirlRenderer,
    ));
}

#[derive(Event)]
pub struct UpdateSwirlParams{
    pub new_params: SwirlParams
}

pub fn update_swirl_params_event(
    mut query: Query<&Handle<SwirlMaterial>, With<SwirlRenderer>>,
    mut materials: ResMut<Assets<SwirlMaterial>>,
    mut ev_reader: EventReader<UpdateSwirlParams>,
) {
    for ev in ev_reader.read() {
        if let Ok(mat_handle) = query.get_single_mut() {
            let material = materials.get_mut(mat_handle).unwrap();
            material.params = ev.new_params.clone();
        }
    }
}

#[derive(Clone, Copy)]
pub struct SwirlRandPreset {
    pub offset_strength: f32,
    //pub fb_rot: f32,
    pub uv_scale: f32,
    pub col_rot: Color,
    pub fb_strength: f32,
}

#[derive(Resource, Default, Clone)]
pub struct SwirlAutomation {
    preset: usize,
    fix_fb_rot: bool,
    fix_pal: bool,
}

pub fn swirl_beat(
    mut query: Query<&Handle<SwirlMaterial>, With<SwirlRenderer>>,
    mut materials: ResMut<Assets<SwirlMaterial>>,
    mut beat_reader: EventReader<BeatEvent>,
    mut ev_writer: EventReader<UpdateSwirlParams>,
    automation: Res<SwirlAutomation>,
) {
    for _ in beat_reader.read() {
        let mat_handle = query.get_single_mut().unwrap();
        let mut material = materials.get_mut(mat_handle).unwrap();

        let mut new_params = SwirlParams::default();

        let mut rng = rand::thread_rng();

        // SDFs
        new_params.hex = rng.gen::<f32>() * 1.1 - 0.1;
        new_params.circle = rng.gen::<f32>() * 1.1 - 0.1;
        new_params.cross = rng.gen::<f32>() * 1.1 - 0.1;
        new_params.cross_radius = rng.gen::<f32>() * 0.4 + 0.1;
        new_params.thiccness = rng.gen::<f32>() * 0.03 + 0.005;
        if !automation.fix_pal {
            new_params.palette = rng.gen::<f32>() * 7.;
        }

        if !automation.fix_fb_rot {
            new_params.fb_rot = rng.gen::<f32>() * 2. - 1.;
        }

        // Presets for remaining settings
        let presets = vec![
            SwirlRandPreset { // green portal
                offset_strength: 0.5,
                //fb_rot: 0.5,
                uv_scale: 1.,
                col_rot: Color::rgba(0.135, 0.882, 0.148, 1.000),
                fb_strength: 0.65,
            },
            SwirlRandPreset { // Blur out
                offset_strength: 0.34,
                //fb_rot: 0.5,
                uv_scale: 0.98,
                col_rot: Color::rgba(0.232, 0.116, 0.430, 0.304),
                fb_strength: 0.54,
            },
            SwirlRandPreset { // Rainbow out
                offset_strength: 0.34,
                //fb_rot: 0.5,
                uv_scale: 0.99,
                col_rot: Color::rgba(0.842, 0.597, 0.547, 0.416),
                fb_strength: 0.56,
            },
            SwirlRandPreset { // Fractal 1
                offset_strength: 0.63,
                //fb_rot: 0.5,
                uv_scale: 0.5,
                col_rot: Color::rgba(0.047, 1.000, 0.017, 1.000),
                fb_strength: 0.19,
            },
            SwirlRandPreset { // Fractal 2
                offset_strength: 0.,
                //fb_rot: 0.5,
                uv_scale: 0.9,
                col_rot: Color::rgba(0.947, 0.703, 0.247, 1.000),
                fb_strength: 0.25,
            },
            SwirlRandPreset { // Fractal 3
                offset_strength: 0.,
                //fb_rot: 0.5,
                uv_scale: 2.,
                col_rot: Color::rgba(0.947, 0.703, 0.247, 1.000),
                fb_strength: 0.24,
            },
            SwirlRandPreset { // Blue Pixelstorm
                offset_strength: 7.44,
                //fb_rot: 0.5,
                uv_scale: 1.01,
                col_rot: Color::rgba(0.000, 0.268, 1.000, 1.000),
                fb_strength: 0.56,
            },
        ];

        let preset = presets.get(automation.preset).unwrap_or(&presets[0]);

        new_params.offset_strength = preset.offset_strength.clone();
        new_params.uv_scale = preset.uv_scale.clone();
        new_params.col_rot = preset.col_rot.clone();
        new_params.fb_strength = preset.fb_strength.clone();

        material.params = new_params;
    }
}

/*pub fn swirl_gui(
    mut query: Query<&Handle<SwirlMaterial>, With<SwirlRenderer>>,
    mut materials: ResMut<Assets<SwirlMaterial>>,
    mut ev_reader: EventReader<UpdateSwirlParams>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Swirl").show(contexts.ctx_mut(), |ui| {

        let mat_handle = query.get_single_mut().unwrap();
        let mut material = materials.get_mut(mat_handle).unwrap();

        ui.label("hex");
        ui.add(egui::DragValue::new(&mut material.params.hex).speed(0.01).clamp_range(-1. ..=1.0));

        ui.label("circle");
        ui.add(egui::DragValue::new(&mut material.params.circle).speed(0.01).clamp_range(-1. ..=1.0));

        ui.label("cross");
        ui.add(egui::DragValue::new(&mut material.params.cross).speed(0.01).clamp_range(-1. ..=1.0));

        ui.label("cross radius");
        ui.add(egui::DragValue::new(&mut material.params.cross_radius).speed(0.01).clamp_range(-1. ..=1.0));

        ui.label("thiccness");
        ui.add(egui::DragValue::new(&mut material.params.thiccness).speed(0.001).clamp_range(0.0 ..= 0.1));

        ui.separator();

        ui.label("offset strength");
        ui.add(egui::DragValue::new(&mut material.params.offset_strength).speed(0.01).clamp_range(-10. ..=10.0));

        ui.label("fb rot");
        ui.add(egui::DragValue::new(&mut material.params.fb_rot).speed(0.01).clamp_range(-1. ..=1.0));

        ui.label("uv_scale");
        ui.add(egui::DragValue::new(&mut material.params.uv_scale).speed(0.01).clamp_range(-1. ..=10.0));


        ui.label("col_rot");
        let mut col_vals = material.params.col_rot.as_rgba_f32();
        ui.color_edit_button_rgba_unmultiplied(&mut col_vals);
        material.params.col_rot = Color::rgba_from_array(col_vals);

        ui.label("fb strength");
        ui.add(egui::DragValue::new(&mut material.params.fb_strength).speed(0.01).clamp_range(0. ..=1.0));

        ui.label("pal");
        ui.add(egui::DragValue::new(&mut material.params.palette).speed(1.).clamp_range(0. ..=6.0));
    });
}*/