use bevy_ecs::reflect::ReflectComponent;
use bevy_reflect::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct GltfExtras {
    #[serde(rename = "ZombrrRigidBody")]
    pub rigid_body: Option<String>,
    #[serde(rename = "ZombrrCollider")]
    pub collider: Option<String>,
    #[serde(rename = "ZombrrDebugColor")]
    pub debug_color: Option<String>,
}
