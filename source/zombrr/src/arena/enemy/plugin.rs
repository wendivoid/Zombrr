use bevy::prelude::*;
use zombrr_core::{ArenaState, ZombrrState};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<super::SpawnEnemy>()
            .add_system_set(
                SystemSet::on_update(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::spawn::spawn_enemy.system())
                    .with_system(super::brain::handle_bline.system())

            );
    }
}
