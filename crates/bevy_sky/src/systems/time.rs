use bevy::prelude::*;

use crate::{SkyMaterial, Sun};

pub fn pass_time(
    time: Res<Time>,
    mut materials: ResMut<Assets<SkyMaterial>>,
    mut query: Query<(&mut Sun, &Handle<SkyMaterial>)>,
) {
    for (mut sun, handle) in query.iter_mut() {
        if sun.active {
            sun.tick(time.delta_seconds_f64());
            let (azimuth, inclination) = sun.get_azimuth_inclination();
            let (azimuth_radians, inclination_radians) = (
                (azimuth.to_radians() - std::f64::consts::PI) as f32,
                inclination.to_radians() as f32,
            );
            let material = materials.get_mut(handle).unwrap();
            material.set_sun_position(inclination_radians, azimuth_radians, sun.distance);
        }
    }
}
