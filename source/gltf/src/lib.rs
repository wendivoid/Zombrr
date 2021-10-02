use std::collections::HashMap;

mod loader;
pub use loader::*;

use bevy_app::prelude::*;
use bevy_asset::{AddAsset, Handle};
use bevy_pbr::prelude::StandardMaterial;
use bevy_reflect::TypeUuid;
use bevy_render::mesh::Mesh;
use bevy_scene::Scene;

mod extras;
pub use extras::GltfExtras;

/// Adds support for GLTF file loading to Apps
#[derive(Default)]
pub struct ZombrrGltfPlugin;

impl Plugin for ZombrrGltfPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_asset_loader::<GltfLoader>()
            .add_asset::<Gltf>()
            .add_asset::<GltfNode>()
            .add_asset::<GltfPrimitive>()
            .add_asset::<GltfMesh>()
            .register_type::<GltfExtras>();
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "7c18660d-303c-49f9-b0ff-3c3098293169"]
pub struct Gltf {
    pub scenes: Vec<Handle<Scene>>,
    pub named_scenes: HashMap<String, Handle<Scene>>,
    pub meshes: Vec<Handle<GltfMesh>>,
    pub named_meshes: HashMap<String, Handle<GltfMesh>>,
    pub materials: Vec<Handle<StandardMaterial>>,
    pub named_materials: HashMap<String, Handle<StandardMaterial>>,
    pub nodes: Vec<Handle<GltfNode>>,
    pub named_nodes: HashMap<String, Handle<GltfNode>>,
    pub default_scene: Option<Handle<Scene>>,
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "8a003a9c-85f3-41c7-b097-a62eafac03fa"]
pub struct GltfNode {
    pub children: Vec<GltfNode>,
    pub mesh: Option<Handle<GltfMesh>>,
    pub transform: bevy_transform::prelude::Transform,
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "06d052b7-9b67-4d14-bad5-470b0ed68940"]
pub struct GltfMesh {
    pub primitives: Vec<GltfPrimitive>,
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "df953586-5696-4fa0-a23c-7ca0a5bb6cc6"]
pub struct GltfPrimitive {
    pub mesh: Handle<Mesh>,
    pub material: Option<Handle<StandardMaterial>>,
}
