use bevy::prelude::*;
use bevy::scene::InstanceId;
use zombrr_core::packages::{WeaponMeta, WeaponRef};

mod finalize;
mod fire;
mod plugin;
mod spawn;
pub use self::plugin::WeaponPlugin;
mod bundle;
pub use self::bundle::WeaponBundle;

pub struct WeaponRoot;
pub struct WeaponEntity;
pub struct UnloadedWeapon(pub InstanceId, WeaponMeta);

#[derive(Debug, Reflect, Copy, Clone)]
pub struct Magazine {
    pub count: usize,
    pub length: usize,
    pub used: usize,
}

impl Magazine {
    pub fn fire(&mut self) -> bool {
        if self.count * self.length <= self.used {
            false
        } else {
            self.used += 1;
            true
        }
    }
}

pub struct SpawnWeapon {
    pub parent: Entity,
    pub weapon: WeaponRef,
}

pub struct FireWeapon {
    pub weapon: Entity,
    pub assailant: Entity,
}
