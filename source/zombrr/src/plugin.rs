use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_loading::LoadingPlugin;
use zombrr_core::{ZombrrState, MenuState, ZombrrPackages};

pub struct ZombrrPlugin;

impl Plugin for ZombrrPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ZombrrPackages>()
            .add_plugin(zombrr_gltf::ZombrrGltfPlugin)
            .add_plugin(crate::devtools::DevToolsPlugin)
            .add_plugin(crate::arena::ArenaPlugin)
            .add_state(ZombrrState::Booting)

            // Physics
            .insert_resource(RapierConfiguration {
                physics_pipeline_active: false,
                query_pipeline_active: false,
                ..Default::default()
            })
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())

            // Booting State
            .add_plugin(LoadingPlugin {
                loading_state: ZombrrState::Booting,
                next_state: ZombrrState::Menu(MenuState::Loading)
            })
            .add_system_set(
                SystemSet::on_update(ZombrrState::Booting)
                    .with_system(bevy_loading::track(super::systems::load_packages.system()))
            );
    }
}
