use bevy::prelude::*;

pub mod character;
pub mod navigatable;
pub mod weapon;

pub struct ArenaControllersPlugin;

impl Plugin for ArenaControllersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(navigatable::NavigatablePlugin)
            .add_plugin(character::CharacterPlugin)
            .add_plugin(weapon::WeaponPlugin)
            .add_plugin(zombrr_health::HealthPlugin)
            .add_plugin(zombrr_damage::DamagePlugin);
    }
}
