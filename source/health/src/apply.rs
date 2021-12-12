use bevy::prelude::*;

use super::*;

pub fn apply_health(
    mut death_events: EventWriter<Death>,
    mut events: EventReader<Damage>,
    mut health_entities: Query<&mut Health>,
) {
    for Damage { value, target, assailant, .. } in events.iter() {
        if let Ok(mut health) = health_entities.get_mut(*target) {
            if health.apply(*value) {
                death_events.send(Death {
                    target: *target,
                    assailant: *assailant,
                });
            }
        }
    }
}
