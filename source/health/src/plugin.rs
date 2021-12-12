use bevy::prelude::*;
use zombrr_core::{ZombrrState, ArenaState};
use zombrr_core::PlayerRoot;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<crate::Health>()
            .add_event::<crate::Death>()
            .add_event::<crate::Damage>()
            .init_resource::<crate::KillCount>()
            .add_system_set(
                SystemSet::on_update(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(crate::apply::apply_health.system())
                    .with_system(crate::death::apply_death.system())
            )
            .add_system_set(
                SystemSet::on_enter(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(add_empty_killcounts.system()),
            );
    }
}

fn add_empty_killcounts(mut res: ResMut<crate::KillCount>, query: Query<Entity, With<PlayerRoot>>) {
    for entity in query.iter() {
        res.0.insert(entity, 0);
    }
}
