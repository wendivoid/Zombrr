use bevy::prelude::*;

pub mod character;
pub mod weapon;
pub mod navigatable;

pub struct ArenaControllersPlugin;

impl Plugin for ArenaControllersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(navigatable::NavigatablePlugin)
            .add_plugin(character::CharacterPlugin)
            .add_plugin(weapon::WeaponPlugin);
    }
}
