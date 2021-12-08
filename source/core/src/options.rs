use crate::packages::{CharacterRef, MapRef, WeaponRef, DisplayRef};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ArenaOptions {
    pub map: MapRef,
    pub enemy: EnemyOptions,
    pub player: PlayerOptions,
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct PlayerOptions {
    pub character: CharacterRef,
    pub weapon: WeaponRef,
    pub display: DisplayRef
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct EnemyOptions {
    pub character: CharacterRef
}
