use bevy::prelude::*;
use bevy_loading::Progress;
use zombrr_core::{ ZombrrPackages, ArenaOptions };

use super::cameras::PlayerCamerasBundle;
use crate::arena::bundles::CharacterBundle;
use crate::arena::bundles::WeaponBundle;

pub fn init_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    options: Res<ArenaOptions>,
    query: Query<(&Name, &Transform), With<crate::arena::map::ArenaGltfMapObject>>
) -> Progress {
    for (name, transform) in query.iter() {
        if name.as_str() == "PlayerSpawn" {
            let weapon = packages.get_weapon(&options.player.weapon).unwrap();
            let weapon_handle = assets.load(weapon.scene_file().as_str());

            let character = packages.get_character(&options.player.character).unwrap();
            let character_handle = assets.load(character.scene_file().as_str());

            let mut char_transform = Transform::identity();
            char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));

            commands.spawn_bundle(super::PlayerBundle::from(*transform))
                .with_children(|parent| {
                    PlayerCamerasBundle::spawn(parent, Transform::from_xyz(0.0, 2.0, 0.7));
                    CharacterBundle::spawn(parent, char_transform, character_handle);
                    WeaponBundle::spawn(parent, Transform::from_xyz(0.1, 1.6, -0.5), weapon_handle);
                });
            return Progress { done: 1, total: 1 };
        }
    }
    Progress {
        done: 0,
        total: 1
    }
}
