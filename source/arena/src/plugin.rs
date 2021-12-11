use bevy::prelude::*;
use zombrr_core::{ArenaOptions, ArenaState, ZombrrState};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ArenaOptions>()
            .add_plugin(zombrr_map::MapPlugin)
            .add_plugin(super::PlayerPlugin)
            .add_plugin(super::EnemyPlugin)
            .add_plugin(super::DisplayPlugin)
            .add_plugin(super::ModesPlugin)
            .add_plugin(super::controllers::ArenaControllersPlugin)
            .add_plugin(bevy_loading::LoadingPlugin {
                loading_state: ZombrrState::Arena(ArenaState::Loading),
                next_state: ZombrrState::Arena(ArenaState::Playing),
            })
            .add_system_set(
                SystemSet::on_enter(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::setup_physics.system()),
            )
            .add_system_set(
                SystemSet::on_exit(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::tear_down_physics.system())
                    .with_system(super::systems::tear_down_user_interface.system())
                    .with_system(super::systems::tear_down_enemies.system()),
            )
            .add_system_set(
                SystemSet::on_pause(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::tear_down_physics.system()),
            )
            .add_system_set(
                SystemSet::on_resume(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::setup_physics.system()),
            );
    }
}
