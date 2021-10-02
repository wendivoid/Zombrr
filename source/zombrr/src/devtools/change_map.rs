use bevy::prelude::*;

use zombrr_core::{ArenaOptions, ZombrrState, MenuState};

pub struct ChangeMap {
    pub namespace: String,
    pub package: String,
    pub name: String
}

pub fn handle_change_map(
    mut events: EventReader<super::ChangeMap>,
    mut options: ResMut<ArenaOptions>,
    mut state: ResMut<State<ZombrrState>>
) {
    for event in events.iter() {
        options.map.namespace = event.namespace.clone();
        options.map.package = event.package.clone();
        options.map.name = event.name.clone();
        state.set(ZombrrState::Menu(MenuState::Loading)).unwrap();
    }
}
