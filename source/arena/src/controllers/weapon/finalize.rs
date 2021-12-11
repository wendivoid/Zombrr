use bevy::prelude::*;

pub fn finalize_weapon_spawns(
    mut commands: Commands,
    query: Query<(Entity, &super::UnloadedWeapon, &Parent)>,
    scene_spawner: Res<SceneSpawner>,
    entities: Query<Entity, Without<Parent>>,
) {
    for (character, super::UnloadedWeapon(scene, meta), parent) in query.iter() {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(*scene) {
            entity_iter.for_each(|entity| {
                if let Ok(entity) = entities.get(entity) {
                    commands
                        .entity(entity)
                        .insert_bundle(super::WeaponBundle::from(meta.clone()));
                    commands.entity(parent.0).push_children(&[entity]);
                } else {
                    commands.entity(entity).insert(zombrr_core::WeaponEntity);
                }
            });
            commands.entity(character).despawn_recursive();
        }
    }
}
