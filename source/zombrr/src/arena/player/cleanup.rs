use bevy::prelude::*;

pub fn cleanup_player_objects(
    mut commands: Commands,
    players: Query<Entity, With<super::PlayerRoot>>
) {
    commands.entity(players.single().unwrap()).despawn_recursive();
}
