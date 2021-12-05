use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<super::SpawnEnemy>()
            .add_system(super::spawn::spawn_enemy.system())
            .add_system(super::brain::handle_bline.system());
    }
}
