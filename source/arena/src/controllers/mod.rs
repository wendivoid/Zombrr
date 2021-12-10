use bevy::prelude::*;

pub mod character;
pub mod damage;
pub mod navigatable;
pub mod tracers;
pub mod weapon;

pub struct ArenaControllersPlugin;

impl Plugin for ArenaControllersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(navigatable::NavigatablePlugin)
            .add_plugin(character::CharacterPlugin)
            .add_plugin(weapon::WeaponPlugin)
            .add_plugin(damage::DamagePlugin)
            .add_plugin(tracers::TracersPlugin);
    }
}
