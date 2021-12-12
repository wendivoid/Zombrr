use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_devtools::DevToolsExt;

mod change_map;
mod change_mode;
pub mod events;
mod skip_menu;

fn main() {
    App::build()
        // Add resources
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AssetServerSettings {
            asset_folder: "../../assets".into(),
        })
        .insert_resource(bevy::log::LogSettings {
            filter: "wgpu=warn".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(WindowDescriptor {
            mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        // Add plugins
        .add_plugins(zombrr::ZombrrPlugins)
        .insert_resource(zombrr_arena::modes::Mode::OneEnemy)
        // Development Tools
        .add_event::<events::ChangeMap>()
        .add_system(change_map::handle_change_map.system())
        .devtools_tool(change_map::change_map())
        .add_event::<events::ChangeMode>()
        .add_system(change_mode::handle_change_mode.system())
        .devtools_tool(change_mode::change_mode())

        .add_system_set(skip_menu::system_set())
        .add_system(toggle_cursor.system())
        .add_startup_system(hide_cursor.system())
        .run()
}

fn hide_cursor(mut windows: ResMut<Windows>) {
    let primary = windows.get_primary_mut().unwrap();
    primary.set_cursor_lock_mode(true);
    primary.set_cursor_visibility(false);
}


fn toggle_cursor(
    keys: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    mut mouse: Query<&mut zombrr_arena::controllers::navigatable::MouseInput>
) {
    if keys.just_pressed(KeyCode::Escape) {
        let primary = windows.get_primary_mut().unwrap();
        if primary.cursor_locked() {
            primary.set_cursor_lock_mode(false);
            primary.set_cursor_visibility(true);
            for mut mouse in mouse.iter_mut() {
                mouse.disabled = true;
            }
        } else {
            primary.set_cursor_lock_mode(true);
            primary.set_cursor_visibility(false);
            for mut mouse in mouse.iter_mut() {
                mouse.disabled = false;
            }
        }
    }
}
