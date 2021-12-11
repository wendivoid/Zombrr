use bevy::prelude::*;

pub fn cleanup_player_objects(
    mut commands: Commands,
    players: Query<Entity, With<zombrr_core::PlayerRoot>>,
) {
    commands
        .entity(players.single().unwrap())
        .despawn_recursive();
}
