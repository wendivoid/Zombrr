use bevy::prelude::*;
use zombrr_core::packages::Color as AssetColor;

pub fn zombrr_color_to_bevy(core: &AssetColor) -> Color {
    match core {
        AssetColor::Rgb(data) => Color::rgb(data[0], data[1], data[2]),
        AssetColor::RgbPre(data) => Color::rgb_u8(data[0], data[1], data[2]),
        AssetColor::Rgba(data) => Color::rgba(data[0], data[1], data[2], data[3]),
        AssetColor::RgbaPre(data) => Color::rgba_u8(data[0], data[1], data[2], data[3]),
        AssetColor::Hex(data) => Color::hex(data).unwrap(),
    }
}
