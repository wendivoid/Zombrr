use bevy::prelude::*;
use zombrr_core::ZombrrPackages;

use crate::arena::SpawnEnemy;
use crate::arena::bundles::CharacterBundle;

pub fn spawn_enemy(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    players: Query<Entity, With<crate::arena::player::PlayerRoot>>,
    mut events: EventReader<SpawnEnemy>,
) {
    for spawn_event in events.iter() {
        let player = players.single().unwrap();
        let character = packages.get_character(&spawn_event.character).unwrap();
        let character_path = character.scene_file();
        debug!(
            "Spawning Enemy\n\t-> Name = {:?}\n\t-> Position = {}\n\t-> Speed = {}\n\t-> Scene File = {}",
            character.name, spawn_event.translation, spawn_event.speed, character_path
        );

        let enemy_transform = Transform::from_translation(spawn_event.translation);

        let mut char_transform = Transform::identity();
        char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));

        commands.spawn_bundle(super::EnemyBundle::new(enemy_transform, spawn_event.speed))
            .insert(super::brain::BLine { to: player })
            .with_children(|parent| {
                let character_handle = assets.load(character_path.as_str());
                CharacterBundle::spawn(parent, char_transform, character_handle);
            });
    }
}
