use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WeaponMeta {
    #[serde(flatten)]
    pub label: Option<String>,
    #[serde(default = "magazine_length")]
    pub magazine_length: usize,
    #[serde(default = "magazine_count")]
    pub magazine_count: usize,
    #[serde(default = "scene_file")]
    pub scene: String,
    #[serde(default = "spaced_tilde")]
    pub description: String
}

fn scene_file() -> String { "scene.glb".into() }
fn spaced_tilde() -> String { " ~ ".into() }
fn magazine_length() -> usize { 10 }
fn magazine_count() -> usize { 8 }
