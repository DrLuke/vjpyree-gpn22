use bevy::ecs::system::SystemParam;
use bevy::prelude::{EventReader, Local, ResMut};
use bevy::utils::default;
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Color32, RichText, Ui, WidgetText};
use crate::anims::meta_tunnelgon::{TunnelgonLaserCycleMetaAnim, TunnelgonLaserFigureEightMetaAnim, TunnelgonLaserRoundTheClockMetaAnim, TunnelgonLaserSweepMetaAnim, TunnelgonRingsBTFMetaAnim, TunnelgonRingsFTBMetaAnim, TunnelgonRingsTrainMetaAnim};
use crate::beat::BeatEvent;


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

#[derive(Default, Copy, Clone)]
pub struct MetaAnimStorage {
    tg: TgMetaAnimStorage,
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
    mut memory: Local<MetaAnimMemory>,
    mut beat_reader: EventReader<BeatEvent>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("Animations GUI")
        .resizable(false)
        .show(ctx, |ui| {
            let button_width = 120.;
            let button_height = 40.;

            ui.heading("Animations");

            let mut settings= match memory.next {
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
        });
}

fn anim_button(ui: &mut Ui, width: f32, height: f32, toggle: &mut bool, text: impl Into<WidgetText>) {
    if ui.add_sized([width, height], egui::SelectableLabel::new(*toggle, text))
        .clicked() {
        *toggle = !(*toggle);
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
        ..default()
    }
}