use bevy::app::App;
use bevy::prelude::{Component, Plugin, Query, Res, Update};
use bevy::time::Time;


pub struct AnimPlugin;

impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (linear_anim_system, pt1_anim_system));
    }
}

pub trait ParameterAnimation {
    fn get_val(&self) -> f32;
    fn get_target(&self) -> f32;
    fn target_reached(&self) -> bool { self.get_val() == self.get_target() }
}

#[derive(Component)]
pub struct LinearAnim {
    pub val: f32,
    pub target: f32,
    pub speed: f32,
}

pub fn linear_anim_system(
    mut query: Query<&mut LinearAnim>,
    time: Res<Time>,
) {
    for mut anim in query.iter_mut() {
        let diff = anim.target - anim.val;
        let step = diff.signum() * time.delta_seconds() * anim.speed;
        if step > diff {
            anim.val += step;
        } else {
            anim.val = anim.target;
        }
    }
}

impl ParameterAnimation for LinearAnim {
    fn get_val(&self) -> f32 { self.val }
    fn get_target(&self) -> f32 { self.target }
}


#[derive(Component)]
pub struct Pt1Anim {
    pub val: f32,
    pub target: f32,
    pub time_constant: f32,
}

impl Default for Pt1Anim {
    fn default() -> Self {
        Self {
            val: 0.,
            target: 1.,
            time_constant: 0.1,
        }
    }
}

impl ParameterAnimation for Pt1Anim {
    fn get_val(&self) -> f32 { self.val }
    fn get_target(&self) -> f32 { self.target }
}

pub fn pt1_anim_system(
    mut query: Query<&mut Pt1Anim>,
    time: Res<Time>,
) {
    for mut anim in query.iter_mut() {
        let step = (anim.target - anim.val) * (time.delta_seconds() / (anim.time_constant + time.delta_seconds()));
        if step.abs() > 0.0001 {
            anim.val += step;
        } else {
            anim.val = anim.target;
        }
    }
}