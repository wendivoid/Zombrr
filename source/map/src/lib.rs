mod cleanup;
mod entities;
mod init;
mod plugin;
mod resources;
mod spawn;
mod active_map;
pub use self::entities::*;
pub use self::plugin::MapPlugin;
pub use self::resources::ArenaMapData;
pub use self::active_map::ActiveMap;

pub mod gltf;

use zombrr_core::packages::Color as AssetColor;

pub(crate) fn zombrr_color_to_bevy(core: &AssetColor) -> bevy_render::color::Color {
    match core {
        AssetColor::Rgb(data) => bevy_render::color::Color::rgb(data[0], data[1], data[2]),
        AssetColor::RgbPre(data) => bevy_render::color::Color::rgb_u8(data[0], data[1], data[2]),
        AssetColor::Rgba(data) => {
            bevy_render::color::Color::rgba(data[0], data[1], data[2], data[3])
        }
        AssetColor::RgbaPre(data) => {
            bevy_render::color::Color::rgba_u8(data[0], data[1], data[2], data[3])
        }
        AssetColor::Hex(data) => bevy_render::color::Color::hex(data).unwrap(),
    }
}
