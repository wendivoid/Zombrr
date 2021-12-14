use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_loading::track;

use zombrr_core::{ArenaState, ZombrrState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(crate::gltf::GltfMapPlugin)
            .add_plugin(bevy_sky::SkyPlugin)
            .add_system_set(
                SystemSet::on_enter(ZombrrState::Arena(ArenaState::Loading))
                    .with_system(super::systems::spawn_arena_map.system()),
            )
            .add_system_set(
                SystemSet::on_update(ZombrrState::Arena(ArenaState::Loading))
                    .with_system(track(super::systems::init_map_objects.system())),
            )
            .add_system_set(
                SystemSet::on_exit(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(super::systems::cleanup_map_objects.system()),
            );
    }
}
