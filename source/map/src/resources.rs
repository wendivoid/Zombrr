use bevy_asset::prelude::*;
use bevy_scene::{prelude::*, InstanceId};

pub struct ArenaMapData {
    pub name: String,
    pub loaded: bool,
    pub scene: Handle<Scene>,
    pub instance_id: InstanceId,
}
