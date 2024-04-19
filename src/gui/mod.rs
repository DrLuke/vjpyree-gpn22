use bevy::app::App;
use bevy::prelude::{Plugin, Update};
use crate::gui::effectors::effectors_gui;

mod effectors;

pub struct GuiPlugin;

impl Plugin for GuiPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, effectors_gui);
    }
}