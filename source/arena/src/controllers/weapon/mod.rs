use bevy::prelude::*;
use bevy::scene::InstanceId;
use zombrr_core::packages::{WeaponMeta, WeaponRef};

mod finalize;
mod fire;
mod plugin;

mod magazine;
pub use self::magazine::Magazine;
mod spawn;
pub use self::plugin::WeaponPlugin;
mod bundle;
pub use self::bundle::WeaponBundle;

pub struct UnloadedWeapon(pub InstanceId, WeaponMeta);

pub struct SpawnWeapon {
    pub parent: Entity,
    pub weapon: WeaponRef,
}

pub struct FireWeapon {
    pub weapon: Entity,
    pub assailant: Entity,
}
