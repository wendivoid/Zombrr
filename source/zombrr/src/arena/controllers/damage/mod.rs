use bevy::prelude::*;
use bevy::utils::HashMap;

mod apply;
mod death;

mod plugin;
pub use self::plugin::DamagePlugin;

#[derive(Default)]
pub struct KillCount(HashMap<Entity, usize>);

#[derive(Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Damage {
    pub value: f32,
    pub total: f32
}

impl Default for Damage {
    fn default() -> Damage {
        Damage { value: 0.0, total: 100.0 }
    }
}

impl Damage {
    pub fn apply(&mut self, value: f32) -> bool {
        if value + self.value >= self.total {
            true
        } else {
            self.value += value;
            false
        }
    }
}

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
