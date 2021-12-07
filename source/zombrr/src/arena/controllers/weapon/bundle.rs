use bevy::prelude::*;

use crate::arena::controllers::weapon::Magazine;

#[derive(Bundle)]
pub struct WeaponBundle {
    pub name: Name,
    pub magazine: Magazine,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub weapon: super::WeaponRoot
}

impl From<Magazine> for WeaponBundle {
    fn from(magazine: Magazine) -> WeaponBundle {
        WeaponBundle {
            magazine,
            name: Name::new("Weapon"),
            transform: Transform::from_xyz(0.1, 1.6, -0.5),
            global_transform: GlobalTransform::identity(),
            weapon: super::WeaponRoot,
        }
    }
}
