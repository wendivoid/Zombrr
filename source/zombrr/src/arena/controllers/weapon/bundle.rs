use bevy::prelude::*;

use crate::arena::controllers::weapon::{Magazine, WeaponMeta};

#[derive(Bundle)]
pub struct WeaponBundle {
    pub name: Name,
    pub weapon: WeaponMeta,
    pub magazine: Magazine,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub root: super::WeaponRoot
}

impl From<WeaponMeta> for WeaponBundle {
    fn from(weapon: WeaponMeta) -> WeaponBundle {
        WeaponBundle {
            magazine: Magazine {
                count: weapon.magazine_count,
                length: weapon.magazine_length,
                used: 0
            },
            weapon,
            name: Name::new("Weapon"),
            transform: Transform::from_xyz(0.1, 1.6, -0.5),
            global_transform: GlobalTransform::identity(),
            root: super::WeaponRoot,
        }
    }
}
