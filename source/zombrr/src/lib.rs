mod systems;
mod devtools;
pub mod arena;
pub mod utils;

mod plugin;
pub use self::plugin::ZombrrPlugin;

pub struct ZombrrPlugins;

impl bevy::app::PluginGroup for ZombrrPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(bevy::log::LogPlugin::default());
        group.add(bevy::core::CorePlugin::default());
        group.add(bevy::transform::TransformPlugin::default());
        group.add(bevy::diagnostic::DiagnosticsPlugin::default());
        group.add(bevy::input::InputPlugin::default());
        group.add(bevy::window::WindowPlugin::default());
        group.add(bevy::asset::AssetPlugin::default());
        group.add(bevy::scene::ScenePlugin::default());
        group.add(bevy::render::RenderPlugin::default());
        group.add(bevy::sprite::SpritePlugin::default());
        group.add(bevy::pbr::PbrPlugin::default());
        group.add(bevy::ui::UiPlugin::default());
        group.add(bevy::text::TextPlugin::default());
        group.add(bevy::audio::AudioPlugin::default());
        group.add(bevy::gilrs::GilrsPlugin::default());
        group.add(bevy::winit::WinitPlugin::default());
        group.add(bevy::wgpu::WgpuPlugin::default());
        group.add(crate::ZombrrPlugin);
    }
}
