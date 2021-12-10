use bevy::prelude::*;
use zombrr_core::{ArenaState, ZombrrState};

mod input;
pub use self::input::{keyboard_input, mouse_button_input, mouse_input, KeyboardInput, MouseInput};

mod collision;
pub use self::collision::navigatable_collision;

mod system;
pub use self::system::navigatable_move;

mod navigatable;
pub use self::navigatable::Navigatable;

pub struct NavigatablePlugin;

impl Plugin for NavigatablePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(ZombrrState::Arena(ArenaState::Playing))
                .with_system(navigatable_move.system())
                .with_system(keyboard_input.system())
                .with_system(mouse_input.system())
                .with_system(mouse_button_input.system())
                .with_system(navigatable_collision.system()),
        );
    }
}
