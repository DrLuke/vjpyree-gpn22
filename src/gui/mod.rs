use bevy::app::App;
use bevy::prelude::{Plugin, Update};
use crate::gui::effectors::effectors_gui;
use crate::gui::elements2d::elements_2d_gui;

mod effectors;
mod elements2d;

pub struct GuiPlugin;

impl Plugin for GuiPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (effectors_gui, elements_2d_gui));
    }
}