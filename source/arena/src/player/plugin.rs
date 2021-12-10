use bevy::prelude::*;
use bevy_loading::track;
use zombrr_core::{ArenaState, ZombrrState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(ZombrrState::Arena(ArenaState::Loading))
                .with_system(track(super::init::init_player.system())),
        )
        .add_system_set(
            SystemSet::on_exit(ZombrrState::Arena(ArenaState::Playing))
                .with_system(super::cleanup::cleanup_player_objects.system()),
        );
    }
}
