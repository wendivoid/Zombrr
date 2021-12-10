use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CharacterMeta {
    #[serde(flatten)]
    pub label: Option<String>,
    #[serde(default = "scene_file")]
    pub scene: String,
    #[serde(default = "spaced_tilde")]
    pub description: String,
}

fn scene_file() -> String {
    "scene.glb".into()
}
fn spaced_tilde() -> String {
    " ~ ".into()
}
