use bevy::ecs::system::SystemParam;
use bevy::prelude::{EventReader, Local, ResMut};
use bevy::utils::default;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Color32, RichText, Ui, WidgetText};
use crate::anims::meta_phys::{PhysAnimMode, PhysMetaAnim};
use crate::anims::meta_tunnelgon::{TunnelgonLaserCycleMetaAnim, TunnelgonLaserFigureEightMetaAnim, TunnelgonLaserRoundTheClockMetaAnim, TunnelgonLaserSweepMetaAnim, TunnelgonRingsBTFMetaAnim, TunnelgonRingsFTBMetaAnim, TunnelgonRingsTrainMetaAnim};
use crate::anims::tubes::TubesWaveAnims;
use crate::beat::BeatEvent;
use crate::physics_hexagon::effectors::{EyesMode, PhysHexSettings};


#[derive(SystemParam)]
pub struct TgMetaAnim<'w> {
    laser_cycle: ResMut<'w, TunnelgonLaserCycleMetaAnim>,
    laser_figure_eight: ResMut<'w, TunnelgonLaserFigureEightMetaAnim>,
    laser_round_the_clock: ResMut<'w, TunnelgonLaserRoundTheClockMetaAnim>,
    laser_sweep: ResMut<'w, TunnelgonLaserSweepMetaAnim>,
    ring_ftb: ResMut<'w, TunnelgonRingsFTBMetaAnim>,
    ring_btf: ResMut<'w, TunnelgonRingsBTFMetaAnim>,
    ring_train: ResMut<'w, TunnelgonRingsTrainMetaAnim>,
}

impl TgMetaAnim<'_> {
    pub fn load_storage(&mut self, storage: TgMetaAnimStorage) {
        self.laser_cycle.enabled = storage.laser_cycle;
        self.laser_figure_eight.enabled = storage.laser_figure_eight;
        self.laser_round_the_clock.enabled = storage.laser_round_the_clock;
        self.laser_sweep.enabled = storage.laser_sweep;
        self.ring_ftb.enabled = storage.ring_ftb;
        self.ring_btf.enabled = storage.ring_btf;
        self.ring_train.enabled = storage.ring_train;
    }
}

#[derive(Default, Copy, Clone)]
pub struct TgMetaAnimStorage {
    laser_cycle: bool,
    laser_figure_eight: bool,
    laser_round_the_clock: bool,
    laser_sweep: bool,
    ring_ftb: bool,
    ring_btf: bool,
    ring_train: bool,
}

#[derive(SystemParam)]
pub struct TubesAnim<'w> {
    wave: ResMut<'w, TubesWaveAnims>,
}

impl TubesAnim<'_> {
    fn load_storage(&mut self, storage: TubesAnimStorage) {
        self.wave.wave = storage.wave;
        self.wave.punch = storage.punch;
        self.wave.punch2 = storage.punch2;
        self.wave.punch3 = storage.punch3;
        self.wave.punch4 = storage.punch4;
        self.wave.sweep_out = storage.sweep_out;
        self.wave.sweep_in = storage.sweep_in;
    }
}

#[derive(Default, Copy, Clone)]
pub struct TubesAnimStorage {
    wave: usize,
    punch: bool,
    punch2: bool,
    punch3: bool,
    punch4: bool,
    sweep_out: bool,
    sweep_in: bool,
}

#[derive(SystemParam)]
pub struct PhysAnim<'w> {
    phys_meta_anim: ResMut<'w, PhysMetaAnim>,
    phys_hex_settings: ResMut<'w, PhysHexSettings>,
}

impl PhysAnim<'_> {
    pub fn load_storage(&mut self, phys_anim_storage: PhysAnimStorage) {
        self.phys_meta_anim.anim_mode = phys_anim_storage.anim_mode;
        self.phys_hex_settings.eye_count = phys_anim_storage.eye_count;
        self.phys_hex_settings.eyes_mode = phys_anim_storage.eyes_mode;
    }
}

#[derive(Default, Copy, Clone)]
pub struct PhysAnimStorage {
    pub anim_mode: PhysAnimMode,
    pub eye_count: usize,
    pub eyes_mode: EyesMode,
}

#[derive(Default, Copy, Clone)]
pub struct MetaAnimStorage {
    tg: TgMetaAnimStorage,
    tubes: TubesAnimStorage,
    phys: PhysAnimStorage,
}

#[derive(Default)]
pub struct MetaAnimMemory {
    current: MetaAnimStorage,
    next: Option<MetaAnimStorage>,
    edit_direct: bool,
    next_on_beat: bool,
    memory: Vec<MetaAnimMemory>,
}

pub fn anim_gui(
    mut contexts: EguiContexts,
    mut tg: TgMetaAnim,
    mut tubes: TubesAnim,
    mut phys: PhysAnim,
    mut memory: Local<MetaAnimMemory>,
    mut beat_reader: EventReader<BeatEvent>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("Animations GUI")
        .resizable(false)
        .show(ctx, |ui| {
            let button_width = 120.;
            let button_height = 30.;

            ui.heading("Animations");

            let mut settings = match memory.next {
                None => { &mut memory.current }
                Some(_) => { memory.next.as_mut().unwrap() }
            };

            ui.separator();
            ui.label("Laser");

            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut settings.tg.laser_cycle, "Laser Cycle");
                anim_button(ui, button_width, button_height, &mut settings.tg.laser_figure_eight, "Laser Fig8");
            });
            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut settings.tg.laser_round_the_clock, "Laser Clock");
                anim_button(ui, button_width, button_height, &mut settings.tg.laser_sweep, "Laser Sweep");
            });

            ui.separator();
            ui.label("Rings");

            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut settings.tg.ring_ftb, "FTB");
                anim_button(ui, button_width, button_height, &mut settings.tg.ring_btf, "BTF");
            });

            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut settings.tg.ring_train, "Train");
            });

            ui.separator();
            ui.label("Tubes");
            ui.horizontal(|ui| {
                tubes_button(ui, button_width, button_height, &mut settings.tubes.wave, 1, "Wave Out");
                tubes_button(ui, button_width, button_height, &mut settings.tubes.wave, 2, "Wave In");
            });
            ui.horizontal(|ui| {
                tubes_button(ui, button_width, button_height, &mut settings.tubes.wave, 3, "Blocky out");
                tubes_button(ui, button_width, button_height, &mut settings.tubes.wave, 4, "Blocky in");
            });
            ui.horizontal(|ui| {
                tubes_button(ui, button_width, button_height, &mut settings.tubes.wave, 5, "Noise 1");
                tubes_button(ui, button_width, button_height, &mut settings.tubes.wave, 6, "Noise 2");
            });
            ui.horizontal(|ui| {
                tubes_button(ui, button_width, button_height, &mut settings.tubes.wave, 0, "Wave Off");
                anim_button(ui, button_width, button_height, &mut settings.tubes.punch, "Punch1");
            });
            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut settings.tubes.sweep_out, "Sweep out");
                anim_button(ui, button_width, button_height, &mut settings.tubes.punch2, "Punch2");
            });
            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut settings.tubes.punch3, "Punch3");
                anim_button(ui, button_width, button_height, &mut settings.tubes.punch4, "Punch4");
            });

            ui.separator();
            ui.heading("Eyes");

            let e_width = 60.;
            ui.horizontal(|ui| {
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.anim_mode, PhysAnimMode::None, "Off");
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.anim_mode, PhysAnimMode::PushPull, "PushPull");
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.anim_mode, PhysAnimMode::Sides, "Sides");
            });
            ui.horizontal(|ui| {
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.anim_mode, PhysAnimMode::Push, "Push");
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.anim_mode, PhysAnimMode::Pull, "Pull");
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.anim_mode, PhysAnimMode::ContPull, "ContPull");
            });
            ui.horizontal(|ui| {
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.eyes_mode, EyesMode::None, "None");
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.eyes_mode, EyesMode::Crazy, "Crazy");
                phys_anim_mode_button(ui, e_width, button_height, &mut settings.phys.eyes_mode, EyesMode::Stare, "Stare");
            });
            ui.add(egui::DragValue::new(&mut settings.phys.eye_count).speed(1).clamp_range(0..=30));


            ui.separator();
            ui.label("Presets");
            if ui.button("Preset 1").clicked() {
                memory.next = Some(preset1())
            }

            ui.separator();
            ui.heading("Memory");

            ui.horizontal(|ui| {
                if let Some(_) = memory.next {
                    ui.label(RichText::new("CURRENT SETTINGS UNLOADED").color(Color32::RED).strong());
                } else {
                    ui.label(RichText::new("Live Settings").color(Color32::GREEN).strong());
                }
            });
            ui.horizontal(|ui| {
                if ui.add_sized([button_width, button_height], egui::Button::new("Prepare"))
                    .clicked() {
                    memory.next = Some(memory.current);
                };
                anim_button(ui, button_width, button_height, &mut memory.next_on_beat, "Load on beat");
            });
            ui.horizontal(|ui| {
                if ui.add_sized([button_width, button_height], egui::Button::new("Load"))
                    .clicked() {
                    if let Some(next) = memory.next {
                        memory.current = next;
                        memory.next = None;
                    }
                };
                if ui.add_sized([button_width, button_height], egui::Button::new("Discard"))
                    .clicked() {
                    if let Some(_) = memory.next {
                        memory.next = None;
                    }
                };
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

            // If next on beat is activated, switch next to current on beat
            if let Some(next) = memory.next {
                if memory.next_on_beat {
                    for _ in beat_reader.read() {
                        memory.current = next;
                        memory.next = None;
                        memory.next_on_beat = false;
                    }
                }
            }

            // Load current settings
            tg.load_storage(memory.current.tg);
            tubes.load_storage(memory.current.tubes);
            phys.load_storage(memory.current.phys);
        });
}

fn anim_button(ui: &mut Ui, width: f32, height: f32, toggle: &mut bool, text: impl Into<WidgetText>) {
    if ui.add_sized([width, height], egui::SelectableLabel::new(*toggle, text))
        .clicked() {
        *toggle = !(*toggle);
    };
}

fn tubes_button(ui: &mut Ui, width: f32, height: f32, wave: &mut usize, wave_set: usize, text: impl Into<WidgetText>) {
    if ui.add_sized([width, height], egui::SelectableLabel::new(*wave == wave_set, text))
        .clicked() {
        *wave = wave_set;
    };
}

fn phys_anim_mode_button<T>(ui: &mut Ui, width: f32, height: f32, set: &mut T, set_to: T, text: impl Into<WidgetText>)
    where T: PartialEq
{
    if ui.add_sized([width, height], egui::SelectableLabel::new(*set == set_to, text))
        .clicked() {
        *set = set_to;
    };
}

fn preset1() -> MetaAnimStorage {
    MetaAnimStorage {
        tg: TgMetaAnimStorage {
            laser_round_the_clock: true,
            ring_ftb: true,
            ring_btf: true,
            ..default()
        },
        tubes: TubesAnimStorage {
            wave: 1,
            ..default()
        },
        ..default()
    }
}