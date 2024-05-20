use bevy::ecs::system::SystemParam;
use bevy::prelude::{Local, ResMut};
use bevy_egui::{egui, EguiContexts};
use bevy_egui::egui::{Ui, WidgetText};
use crate::anims::meta_tunnelgon::{TunnelgonLaserCycleMetaAnim, TunnelgonLaserFigureEightMetaAnim, TunnelgonLaserRoundTheClockMetaAnim, TunnelgonLaserSweepMetaAnim, TunnelgonRingsBTFMetaAnim, TunnelgonRingsFTBMetaAnim, TunnelgonRingsTrainMetaAnim};

/*
## Laser
* TunnelgonLaserCycleMetaAnim
* TunnelgonLaserFigureEightMetaAnim
* TunnelgonLaserRoundTheClockMetaAnim
* TunnelgonLaserSweepMetaAnim
*/
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

pub fn anim_gui(
    mut contexts: EguiContexts,
    mut tg: TgMetaAnim,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("Animations GUI")
        .resizable(false)
        .show(ctx, |ui| {
            let button_width = 120.;
            let button_height = 40.;

            ui.heading("Animations");

            ui.separator();
            ui.label("Laser");

            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut tg.laser_cycle.enabled, "Laser Cycle");
                anim_button(ui, button_width, button_height, &mut tg.laser_figure_eight.enabled, "Laser Fig8");
            });
            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut tg.laser_round_the_clock.enabled, "Laser Clock");
                anim_button(ui, button_width, button_height, &mut tg.laser_sweep.enabled, "Laser Sweep");
            });

            ui.separator();
            ui.label("Rings");

            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut tg.ring_ftb.enabled, "FTB");
                anim_button(ui, button_width, button_height, &mut tg.ring_btf.enabled, "BTF");
            });

            ui.horizontal(|ui| {
                anim_button(ui, button_width, button_height, &mut tg.ring_train.enabled, "Train");
            });

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}

fn anim_button(ui: &mut Ui, width: f32, height: f32, toggle: &mut bool, text: impl Into<WidgetText>) {
    if ui.add_sized([width, height], egui::SelectableLabel::new(*toggle, text))
        .clicked() {
        *toggle = !(*toggle);
    };
}