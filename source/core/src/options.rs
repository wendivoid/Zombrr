use crate::packages::{CharacterRef, MapRef, WeaponRef};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ArenaOptions {
    pub map: MapRef,
    pub enemy: EnemyOptions,
    pub player: PlayerOptions,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct PlayerOptions {
    pub character: CharacterRef,
    pub weapon: WeaponRef
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct EnemyOptions {
    pub character: CharacterRef
}
