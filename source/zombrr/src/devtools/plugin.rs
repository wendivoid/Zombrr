use bevy::prelude::*;
use zombrr_core::ZombrrState;
use bevy_devtools::DevToolsExt;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<super::ChangeMap>()
            .add_plugin(bevy_devtools::DevToolsPlugin::<ZombrrState>::default())
            .add_plugin(bevy_hilt::HiltDebugPlugin)
            .devtools_setting(super::physics::settings())
            .devtools_active_panel(0)
            .devtools_enabled()
            .devtools_panel_index(0, super::panel::change_arena_map())
            .add_system(super::change_map::handle_change_map.system())
            .add_system(super::toggle::toggle.system())
            .add_system(super::physics::show_colliders_changed.system())
            .add_system(super::physics::show_render_pass_changed.system());
    }
}
