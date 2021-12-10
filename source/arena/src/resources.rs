use bevy::prelude::*;
use bevy::scene::InstanceId;

#[derive(Default)]
pub struct ArenaResources {
    pub map: Option<ArenaMapData>
}

pub struct ArenaMapData {
    pub name: String,
    pub loaded: bool,
    pub scene: Handle<Scene>,
    pub instance_id: InstanceId
}
