mod hexagon;
mod eyes;

use bevy::prelude::*;
use crate::hexagon::HexagonPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HexagonPlugin)
        .run();
}