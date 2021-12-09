use bevy::prelude::*;

use super::*;

pub fn animate_tracers(
    mut commands: Commands,
    sys_time: Res<Time>,
    mut query: Query<(Entity, &mut TracerTimer)>
) {
    for (entity, mut time) in query.iter_mut() {
        time.0.tick(sys_time.delta());
        if time.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
