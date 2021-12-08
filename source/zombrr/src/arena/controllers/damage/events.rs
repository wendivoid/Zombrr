pub struct Death {
    pub target: bevy::prelude::Entity,
    pub assailant: bevy::prelude::Entity
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SustainedDamage {
    pub value: f32,
    pub target: bevy::prelude::Entity,
    pub assailant: bevy::prelude::Entity
}
