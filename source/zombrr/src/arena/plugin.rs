use bevy::prelude::*;
use zombrr_core::{ArenaState, ArenaOptions, ZombrrState};

use super::ArenaResources;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArenaOptions>()
            .init_resource::<ArenaResources>()
            .add_plugin(super::MapPlugin)
            .add_plugin(super::PlayerPlugin)
            .add_plugin(super::EnemyPlugin)
            .add_plugin(bevy_sky::SkyPlugin)
            .add_plugin(super::controllers::ArenaControllersPlugin)
            .add_plugin(bevy_loading::LoadingPlugin {
                loading_state: ZombrrState::Arena(ArenaState::Loading),
                next_state: ZombrrState::Arena(ArenaState::Playing)
            })
            .add_system_set(
                SystemSet::on_enter(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::setup_physics.system())
            )
            .add_system_set(
                SystemSet::on_exit(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::tear_down_physics.system())
            )
            .add_system_set(
                SystemSet::on_pause(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::tear_down_physics.system())
            )
            .add_system_set(
                SystemSet::on_resume(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::setup_physics.system())
            );
    }
}
