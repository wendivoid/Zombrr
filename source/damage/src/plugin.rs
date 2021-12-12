use bevy::prelude::*;
use zombrr_core::{ArenaState, ZombrrState};

use super::*;

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
                SystemSet::on_update(ZombrrState::Arena(ArenaState::Playing))
                    .with_system(bullet_holes::draw_bullet_holes.system())
                    .with_system(bullet_holes::remove_bullet_holes.system())
            );
    }
}
