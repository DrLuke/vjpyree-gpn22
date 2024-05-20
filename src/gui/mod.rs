use bevy::app::App;
use bevy::prelude::{Plugin, Update};
use crate::GuiUpdate;
use crate::gui::anims::anim_gui;
use crate::gui::effectors::effectors_gui;
use crate::gui::elements2d::elements_2d_gui;
use crate::gui::left_panel::{BeatMute, left_panel};

mod effectors;
mod elements2d;
mod anims;
mod left_panel;

pub struct GuiPlugin;

impl Plugin for GuiPlugin{
    fn build(&self, app: &mut App) {
        app.insert_resource(BeatMute::default());
        app.add_systems(GuiUpdate, (effectors_gui, elements_2d_gui, anim_gui, left_panel));
    }
}