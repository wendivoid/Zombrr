use bevy::prelude::*;
use zombrr_core::ZombrrPackages;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<super::SpawnWeapon>()
            .add_event::<super::FireWeapon>()
            .add_system(handle_weapon_spawns.system())
            .add_system(finalize_weapon_spawns.system())
            .add_system(super::input::handle_fire_input.system())
            .add_system(super::fire::handle_fire_weapon.system());
    }
}

pub fn handle_weapon_spawns(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut events: EventReader<super::SpawnWeapon>
) {
    for super::SpawnWeapon { parent, weapon } in events.iter() {
        let weapon = packages.get_weapon(weapon).unwrap();
        let scene_handle = assets.load(weapon.scene_file().as_str());
        let instance_id = scene_spawner.spawn(scene_handle);

        let mut char_transform = Transform::identity();
        char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));

        commands.entity(*parent)
            .insert_bundle((
                char_transform,
                GlobalTransform::identity(),
            ))
            .with_children(|parent| {
                parent.spawn().insert(super::UnloadedWeapon(instance_id,
                super::Magazine {
                    count: weapon.meta.magazine_count,
                    length: weapon.meta.magazine_length,
                    used: 0,
                }));
            });
    }
}

pub fn finalize_weapon_spawns(
    mut commands: Commands,
    query: Query<(Entity, &super::UnloadedWeapon, &Parent)>,
    scene_spawner: Res<SceneSpawner>,
    entities: Query<Entity, Without<Parent>>,
) {
    for (character, super::UnloadedWeapon(scene, magazine), parent) in query.iter() {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(*scene) {
            entity_iter.for_each(|entity| {
                if let Ok(entity) = entities.get(entity) {
                    commands.entity(entity).insert_bundle(super::WeaponBundle::from(*magazine));
                    commands.entity(parent.0).push_children(&[entity]);
                } else {
                    commands.entity(entity).insert(super::WeaponEntity);
                }
            });
            commands.entity(character).despawn_recursive();
        }
    }
}
