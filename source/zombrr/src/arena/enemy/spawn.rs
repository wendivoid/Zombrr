use bevy::prelude::*;
use zombrr_core::{ZombrrPackages, ArenaOptions, ZombrrObject};

use crate::arena::SpawnEnemy;
use crate::arena::controllers::character::SpawnCharacter;

pub fn spawn_enemy(
    mut commands: Commands,
    options: Res<ArenaOptions>,
    packages: Res<ZombrrPackages>,
    mut events: EventReader<SpawnEnemy>,
    mut spawn_characters: EventWriter<SpawnCharacter>,
    players: Query<Entity, With<crate::arena::player::PlayerRoot>>,
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

        let entity = commands.spawn_bundle(super::EnemyBundle::new(enemy_transform, spawn_event.speed))
            .insert(super::brain::BLine { to: player })
            .insert(ZombrrObject::Enemy)
            .id();
        spawn_characters.send(SpawnCharacter {
            parent: entity,
            character: options.player.character.clone()
        });
    }
}
