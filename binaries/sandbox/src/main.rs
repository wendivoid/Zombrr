use bevy::prelude::*;
use bevy::asset::AssetServerSettings;

mod skip_menu;

fn main() {
    App::build()
        // Add resources
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AssetServerSettings {
            asset_folder: "../../assets".into()
        })
        .insert_resource(bevy::log::LogSettings {
            filter: "wgpu=warn".into(),
            level: bevy::log::Level::DEBUG
        })
        // Add plugins
        .add_plugins(zombrr::ZombrrPlugins)

        // Temperary Systems
        .add_system_set(skip_menu::system_set())

        .run()
}
