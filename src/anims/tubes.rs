use std::f32::consts::PI;
use std::future::join;
use bevy::prelude::{Color, GlobalTransform, Query, Real, Time};
use bevy_defer::{AsyncAccess, AsyncFailure, world};
use crate::anims::AnimColors;
use crate::physics_hexagon::lights::led_tube::LedTubeLed;

pub async fn anim1(speed: f32) -> Result<(), AsyncFailure> {
    let tubes = world().query::<(&mut LedTubeLed, &GlobalTransform)>();
    let time_resource = world().resource::<Time<Real>>();
    let anim_colors = world().resource::<AnimColors>();

    loop {
        let (time, ac) = join!(
            time_resource.get(|t| t.elapsed_seconds()),
            anim_colors.get(|ac| ac.clone())
        ).await;
        let time: f32 = time.unwrap();
        let ac: AnimColors = ac.unwrap();

        tubes.for_each(move |(mut ltl, gt)| {
            let x = gt.translation().x;
            ltl.color = ac.primary * (x * 0.005 - time * speed * x.signum()).sin().powf(8.) +
                ac.secondary * (x * 0.005 - time * speed * x.signum() + PI / 2.).sin().powf(8.);
        }).await;
    }

    Ok(())
}

pub async fn center_sweep_out(speed: f32, start_time: f32) -> Result<(), AsyncFailure> {
    let tubes = world().query::<(&mut LedTubeLed, &GlobalTransform)>();
    let time_resource = world().resource::<Time<Real>>();
    let anim_colors = world().resource::<AnimColors>();

    loop {
        let (time, ac) = join!(
            time_resource.get(|t| t.elapsed_seconds()),
            anim_colors.get(|ac| ac.clone())
        ).await;
        let time: f32 = time.unwrap() - start_time;
        let ac: AnimColors = ac.unwrap();

        tubes.for_each(move |(mut ltl, gt)| {
            let x = gt.translation().x;
            // Glitter
            //ltl.color = ltl.color + ac.primary * (x * 0.05 - time * speed * x.signum()).sin().powf(16.);
            ltl.color = ltl.color + ac.primary * (x * 0.5 - time * speed * x.signum()).sin().powf(16.) * 5.;
        }).await;
        if time > 0.5 { break; }
    }

    Ok(())
}