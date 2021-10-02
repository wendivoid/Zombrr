use serde::{ Serialize, Deserialize };

use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MapData {
    Gltf { path: PathBuf }
}
