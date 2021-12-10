use bevy::prelude::*;
use zombrr_core::ZombrrPackages;

pub fn handle_weapon_spawns(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut events: EventReader<super::SpawnWeapon>,
) {
    for super::SpawnWeapon { parent, weapon } in events.iter() {
        let weapon = packages.get_weapon(weapon).unwrap();
        let scene_handle = assets.load(weapon.scene_file().as_str());
        let instance_id = scene_spawner.spawn(scene_handle);

        let mut char_transform = Transform::identity();
        char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));

        commands
            .entity(*parent)
            .insert_bundle((char_transform, GlobalTransform::identity()))
            .with_children(|parent| {
                parent
                    .spawn()
                    .insert(super::UnloadedWeapon(instance_id, weapon.meta.clone()));
            });
    }
}
