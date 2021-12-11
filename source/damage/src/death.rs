use bevy::prelude::*;
use zombrr_core::EnemyRoot;

pub fn apply_death(
    mut commands: Commands,
    mut deaths: EventReader<super::Death>,
    mut killcounts: ResMut<super::KillCount>,
    entities: Query<Entity, (With<super::Health>, With<EnemyRoot>)>,
) {
    for super::Death { target, assailant } in deaths.iter() {
        if let Ok(entity) = entities.get(*target) {
            commands.entity(entity).despawn_recursive();
            if let Some(score) = killcounts.0.get_mut(&assailant) {
                *score += 1;
            }
        }
    }
}
