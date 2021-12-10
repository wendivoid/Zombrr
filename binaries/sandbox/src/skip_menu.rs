use bevy::prelude::*;
use zombrr_core::{ArenaState, MenuState, ZombrrState};

pub fn system_set() -> SystemSet {
    SystemSet::on_update(ZombrrState::Menu(MenuState::Loading))
        .with_system(skip_menu_system.system())
}

pub fn skip_menu_system(mut state: ResMut<State<ZombrrState>>) {
    state.set(ZombrrState::Arena(ArenaState::Loading)).unwrap();
}
