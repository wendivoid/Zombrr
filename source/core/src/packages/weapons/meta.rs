use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WeaponMeta {
    #[serde(flatten)]
    pub label: Option<String>,
    #[serde(default = "scene_file")]
    pub scene: String,
    #[serde(default = "spaced_tilde")]
    pub description: String,
    #[serde(default = "default_damage")]
    pub damage: f32,
    pub action: WeaponAction,
    pub magazine: Magazine,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum WeaponAction {
    Ray {
        #[serde(default = "default_range")]
        range: f32,
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Magazine {
    #[serde(default = "magazine_length")]
    pub length: usize,
    #[serde(default = "magazine_count")]
    pub count: usize
}

fn scene_file() -> String {
    "scene.glb".into()
}
fn spaced_tilde() -> String {
    " ~ ".into()
}
fn magazine_length() -> usize {
    10
}
fn magazine_count() -> usize {
    8
}
fn default_damage() -> f32 {
    50.0
}
fn default_range() -> f32 {
    1000.0
}
