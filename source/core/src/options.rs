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

#[derive(Debug, PartialEq, Clone)]
pub struct EnemyOptions {
    pub speed: f32,
    pub character: CharacterRef
}

impl Default for EnemyOptions {
    fn default() -> EnemyOptions {
        EnemyOptions {
            speed: 0.2,
            character: Default::default()
        }
    }
}
