use bevy_ecs::prelude::*;
use bevy_transform::prelude::*;

pub fn cleanup_map_objects(
    mut commands: Commands,
    map_roots: Query<Entity, With<super::ArenaMapRoot>>,
    skybox: Query<Entity, With<super::ArenaMapSkyBox>>,
) {
    commands
        .entity(map_roots.single().unwrap())
        .despawn_recursive();
    commands
        .entity(skybox.single().unwrap())
        .despawn_recursive();
}
