use bevy::prelude::*;
use bevy::scene::InstanceId;
use zombrr_core::packages::CharacterRef;

mod finalize;
mod spawn;

mod plugin;
pub use self::plugin::CharacterPlugin;

mod bundle;
pub use self::bundle::CharacterBundle;

pub struct CharacterRoot;
pub struct CharacterEntity;
pub struct UnloadedCharacter(pub InstanceId);

pub struct SpawnCharacter {
    pub parent: Entity,
    pub character: CharacterRef,
}
