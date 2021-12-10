mod spawn;

pub mod brain;

mod plugin;
pub use self::plugin::EnemyPlugin;

mod bundle;
pub use self::bundle::EnemyBundle;

use zombrr_core::packages::CharacterRef;

pub struct SpawnEnemy {
    pub speed: f32,
    pub translation: bevy::math::Vec3,
    pub character: CharacterRef,
}
pub struct EnemyRoot;
