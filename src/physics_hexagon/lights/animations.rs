use std::f32::consts::PI;
use bevy::prelude::{Color, GlobalTransform, Query, Res, Time};
use crate::physics_hexagon::lights::led_tube::LedTubeLed;

pub fn wave_animation_system(
    mut query: Query<(&mut LedTubeLed, &GlobalTransform)>,
    time: Res<Time>,
) {
    for (mut ltl, gt) in query.iter_mut() {
        let x  = gt.translation().x;
        ltl.color = Color::rgb(
            (x*0.005 - time.elapsed_seconds()*2.*x.signum()).sin().powf(8.),
            0.05,
            0.2 + (x*0.005 - time.elapsed_seconds()*2.*x.signum() + PI/2.).sin().powf(8.) * 0.1,
        );
    }
}