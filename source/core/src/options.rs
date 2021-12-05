use crate::packages::{CharacterRef, MapRef, WeaponRef};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct ArenaOptions {
    pub map: MapRef,
    pub player: PlayerOptions
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct PlayerOptions {
    pub character: CharacterRef,
    pub weapon: WeaponRef
}
