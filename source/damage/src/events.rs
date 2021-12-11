pub struct Death {
    pub target: bevy::prelude::Entity,
    pub assailant: bevy::prelude::Entity,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SustainedDamage {
    pub value: f32,
    pub surface_normal: bevy::prelude::Vec3,
    pub point: bevy::prelude::Vec3,
    pub target: bevy::prelude::Entity,
    pub assailant: bevy::prelude::Entity,
}
