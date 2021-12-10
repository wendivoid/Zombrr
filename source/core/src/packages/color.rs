use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    Rgb([f32; 3]),
    RgbPre([u8; 3]),
    Rgba([f32; 4]),
    RgbaPre([u8; 4]),
    Hex(String),
}
