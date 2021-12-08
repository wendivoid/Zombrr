use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<super::SpawnWeapon>()
            .add_event::<super::FireWeapon>()
            .add_system(super::spawn::handle_weapon_spawns.system())
            .add_system(super::finalize::finalize_weapon_spawns.system())
            .add_system(super::fire::handle_fire_weapon.system());
    }
}
