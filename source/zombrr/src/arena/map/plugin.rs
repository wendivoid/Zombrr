use bevy::prelude::*;
use bevy_loading::track;
use zombrr_core::{ZombrrState, ArenaState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
                SystemSet::on_enter(ZombrrState::Arena(ArenaState::Loading))
                    .with_system(super::spawn::spawn_arena.system())
            )
            .add_system_set(
                SystemSet::on_update(ZombrrState::Arena(ArenaState::Loading))
                    .with_system(track(super::init::init_map_objects.system()))
            )
            .add_system_set(
                SystemSet::on_exit(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::cleanup::cleanup_map_objects.system())
            );
    }
}
