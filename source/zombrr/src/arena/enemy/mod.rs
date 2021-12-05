mod spawn;

pub mod brain;

mod plugin;
pub use self::plugin::EnemyPlugin;

use zombrr_core::packages::CharacterRef;

pub struct SpawnEnemy {
    pub translation: bevy::math::Vec3,
    pub character: CharacterRef
}

pub struct EnemyRoot;
