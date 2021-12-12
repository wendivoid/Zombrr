use bevy::prelude::*;
use zombrr_core::EnemyRoot;

pub struct Death {
    pub target: Entity,
    pub assailant: Entity,
}

pub fn apply_death(
    mut commands: Commands,
    mut deaths: EventReader<Death>,
    mut killcounts: ResMut<crate::KillCount>,
    entities: Query<Entity, (With<crate::Health>, With<EnemyRoot>)>,
) {
    for Death { target, assailant } in deaths.iter() {
        if let Ok(entity) = entities.get(*target) {
            commands.entity(entity).despawn_recursive();
            if let Some(score) = killcounts.0.get_mut(&assailant) {
                *score += 1;
            }
        }
    }
}
