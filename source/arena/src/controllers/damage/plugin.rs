use bevy::prelude::*;
use zombrr_core::{ArenaState, ZombrrState};

use super::*;
use crate::player::PlayerRoot;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<KillCount>()
            .add_event::<SustainedDamage>()
            .add_event::<Death>()
            .register_type::<Damage>()
            .add_system_set(
                SystemSet::on_update(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(apply::apply_health.system())
                    .with_system(death::apply_death.system())
                    .with_system(apply::remove_bullet_decals.system())
            )
            .add_system_set(
                SystemSet::on_enter(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(add_empty_killcounts.system()),
            );
    }
}

fn add_empty_killcounts(mut res: ResMut<KillCount>, query: Query<Entity, With<PlayerRoot>>) {
    for entity in query.iter() {
        res.0.insert(entity, 0);
    }
}
