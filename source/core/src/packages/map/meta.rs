use serde::{Deserialize, Serialize};

use super::MapData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MapMeta {
    pub map: MapData,
    #[serde(flatten)]
    pub label: Option<String>,
    #[serde(default = "fifty")]
    pub priority: usize,
    #[serde(default = "spaced_tilde")]
    pub description: String,
    #[serde(default)]
    pub ambient_light: AmbientLight,
    #[serde(default)]
    pub sky: super::Sky,
}

fn fifty() -> usize {
    5
}
fn spaced_tilde() -> String {
    " ~ ".into()
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmbientLight {
    pub color: crate::packages::Color,
    pub brightness: f32,
}

impl Default for AmbientLight {
    fn default() -> AmbientLight {
        AmbientLight {
            color: crate::packages::Color::Rgb([1.0, 1.0, 1.0]),
            brightness: 0.5,
        }
    }
}
