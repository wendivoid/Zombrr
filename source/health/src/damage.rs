use bevy::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Damage {
    pub value: f32,
    pub surface_normal: Vec3,
    pub point: Vec3,
    pub target: Entity,
    pub assailant: Entity,
}
