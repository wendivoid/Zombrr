use bevy::prelude::*;

#[derive(Debug, Reflect, PartialEq, Clone, Copy)]
#[reflect(Component)]
pub struct Health {
    pub value: f32,
    pub total: f32,
}

impl Default for Health {
    fn default() -> Health {
        Health {
            value: 0.0,
            total: 100.0,
        }
    }
}

impl Health {
    pub fn apply(&mut self, value: f32) -> bool {
        if value + self.value >= self.total {
            true
        } else {
            self.value += value;
            false
        }
    }
}
