use bevy::prelude::*;
use zombrr_core::{ArenaState, ZombrrState};

pub struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<super::Mode>().add_system_set(
            SystemSet::on_update(ZombrrState::Arena(ArenaState::Playing))
                .with_system(super::one::maintain_one.system()),
        );
    }
}
