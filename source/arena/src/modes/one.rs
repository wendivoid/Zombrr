use bevy::prelude::*;

use crate::enemy::SpawnEnemy;
use zombrr_core::ArenaOptions;

pub fn maintain_one(
    mut spawned: Local<bool>,
    res: Res<super::Mode>,
    options: Res<ArenaOptions>,
    query: Query<&zombrr_core::EnemyRoot>,
    mut events: EventWriter<SpawnEnemy>,
) {
    match *res {
        super::Mode::OneEnemy => {
            if query.iter().count() == 0 {
                if !*spawned {
                    let spawn_event = SpawnEnemy {
                        translation: Vec3::ZERO,
                        character: options.enemy.character.clone(),
                        speed: options.enemy.speed,
                    };
                    events.send(spawn_event);
                    *spawned = true;
                } else {
                    *spawned = false;
                }
            }
        }
        _ => {}
    }
}
