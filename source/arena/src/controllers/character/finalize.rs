use bevy::prelude::*;

pub fn finalize_character_spawns(
    mut commands: Commands,
    query: Query<(Entity, &super::UnloadedCharacter, &Parent)>,
    scene_spawner: Res<SceneSpawner>,
    entities: Query<Entity, Without<Parent>>,
) {
    for (character, super::UnloadedCharacter(scene), parent) in query.iter() {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(*scene) {
            entity_iter.for_each(|entity| {
                if entities.get(entity).is_ok() {
                    let mut char_transform = Transform::identity();
                    char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
                    commands
                        .entity(entity)
                        .insert_bundle(super::CharacterBundle::from(char_transform));
                    commands.entity(parent.0).push_children(&[entity]);
                } else {
                    commands.entity(entity).insert(super::CharacterEntity);
                }
            });
            commands.entity(character).despawn_recursive();
        }
    }
}
