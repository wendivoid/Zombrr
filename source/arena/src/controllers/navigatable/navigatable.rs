use bevy::prelude::*;
use bevy::reflect::Reflect;

#[derive(Reflect)]
pub struct Navigatable {
    pub speed: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub velocity: Vec3,
    pub on_solid: bool,
}

impl Default for Navigatable {
    fn default() -> Navigatable {
        Navigatable {
            speed: 0.3,
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            velocity: Vec3::ZERO,
            on_solid: true,
        }
    }
}

impl Navigatable {
    pub fn get_look_quat(&self) -> Quat {
        Quat::from_rotation_ypr(self.yaw, self.pitch, self.roll)
    }
}
