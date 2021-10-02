use bevy::prelude::*;
use bevy_devtools::Settings;
use zombrr_core::{ZombrrState, DebugState};

pub fn toggle(
    keys: Res<Input<KeyCode>>,
    mut settings: ResMut<Settings>,
    mut state: ResMut<State<ZombrrState>>
) {
    if keys.just_pressed(KeyCode::F11) {
        if let Some(setting) = settings.get_key_mut(&["devtools", "enabled"]) {
            if let Some(b) = setting.value.as_bool_mut() {
                debug!("Toggling DevTools widget: {}", !*b);
                *b = !*b
            } else {
                error!("DevTools Settings key `devtools -> enabled` is not a bool");
            }
        } else {
            error!("Cant find DevTools Settings key `devtools -> enabled`");
        }
    }

    if keys.just_pressed(KeyCode::F12) {
        if state.current() == &ZombrrState::Debug(DebugState::Paused) {
            state.pop().unwrap();
        } else {
            state.push(ZombrrState::Debug(DebugState::Paused)).unwrap();
        }
    }
}
