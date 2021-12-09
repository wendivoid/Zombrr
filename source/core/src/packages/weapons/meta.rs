use serde::{ Serialize, Deserialize };

use crate::packages::Color;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WeaponMeta {
    #[serde(flatten)]
    pub label: Option<String>,
    #[serde(default = "magazine_length")]
    pub magazine_length: usize,
    #[serde(default = "magazine_count")]
    pub magazine_count: usize,
    #[serde(default = "default_damage")]
    pub damage: f32,
    #[serde(default = "tracer_color")]
    pub tracer_color: Color,
    #[serde(default = "scene_file")]
    pub scene: String,
    #[serde(default = "spaced_tilde")]
    pub description: String
}

fn scene_file() -> String { "scene.glb".into() }
fn spaced_tilde() -> String { " ~ ".into() }
fn magazine_length() -> usize { 10 }
fn magazine_count() -> usize { 8 }
fn default_damage() -> f32 { 50.0 }
fn tracer_color() -> Color { Color::Rgba([0.80, 0.33, 0.0, 0.50]) }
