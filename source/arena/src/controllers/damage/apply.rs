use bevy::prelude::*;

use super::*;

pub fn apply_health(
    mut events: EventReader<SustainedDamage>,
    mut death_events: EventWriter<Death>,
    mut health_entities: Query<&mut Damage>,
) {
    for SustainedDamage {
        target,
        value,
        assailant,
    } in events.iter()
    {
        if let Ok(mut health) = health_entities.get_mut(*target) {
            debug!("Dealing `{}` damge to: Entity({:?})", value, *target);
            if health.apply(*value) {
                death_events.send(Death {
                    target: *target,
                    assailant: *assailant,
                });
            }
        }
    }
}
