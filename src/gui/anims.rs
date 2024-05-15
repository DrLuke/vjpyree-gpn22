use bevy_egui::{egui, EguiContexts};

pub fn anim_gui(
    mut contexts: EguiContexts,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("Animations GUI")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}