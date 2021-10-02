use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use super::Navigatable;

pub struct KeyboardInput;
pub struct MouseInput { pub sensitivity: f32 }

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Navigatable, With<KeyboardInput>>
) {
    for mut navigatable in query.iter_mut() {
        navigatable.velocity = Vec3::ZERO;
        let forward = (navigatable.get_look_quat() * Vec3::new(0.0, 0.0, -1.0) * Vec3::new(1.0, 0.0, 1.0)).normalize().clamp_length_max(navigatable.speed);
        let strafe = forward.cross(Vec3::new(0.0, 1.0, 0.0));
        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            navigatable.velocity += forward;
        }

        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down){
            navigatable.velocity -= forward;
        }

        if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
            navigatable.velocity -= strafe;
        }

        if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
            navigatable.velocity += strafe;
        }

        if navigatable.on_solid && keys.just_pressed(KeyCode::Space) {
            navigatable.velocity.y += 4.0;
        }
    }
}

pub fn mouse_input(
    time: Res<Time>,
    mut mousemotion: EventReader<MouseMotion>,
    mut query: Query<(&mut Navigatable, &MouseInput)>
) {
    for (mut navigatable, mouse) in query.iter_mut() {
        let delta_s = time.delta_seconds();
        let mut delta_m = Vec2::ZERO;

        for event in mousemotion.iter() {
            delta_m += event.delta;
        }

        if !delta_m.is_nan() {
            navigatable.pitch = (navigatable.pitch - (delta_m.y * mouse.sensitivity * delta_s).to_radians()).clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
            navigatable.yaw += -(delta_m.x * mouse.sensitivity * delta_s).to_radians();
        }
    }
}