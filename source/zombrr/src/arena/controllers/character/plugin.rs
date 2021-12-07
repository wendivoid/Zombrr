use bevy::prelude::*;
use zombrr_core::ZombrrPackages;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<super::SpawnCharacter>()
            .add_system(handle_character_spawns.system())
            .add_system(finalize_character_spawns.system());
    }
}

pub fn handle_character_spawns(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut events: EventReader<super::SpawnCharacter>
) {
    for super::SpawnCharacter { parent, character } in events.iter() {
        let character = packages.get_character(character).unwrap();
        let scene_handle = assets.load(character.scene_file().as_str());
        let instance_id = scene_spawner.spawn(scene_handle);

        let mut char_transform = Transform::identity();
        char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));

        commands.entity(*parent)
            .insert(char_transform)
            .insert(GlobalTransform::identity())
            .with_children(|parent| {
                parent.spawn().insert(super::UnloadedCharacter(instance_id));
            });
    }
}

pub fn finalize_character_spawns(
    mut commands: Commands,
    query: Query<(Entity, &super::UnloadedCharacter, &Parent)>,
    scene_spawner: Res<SceneSpawner>,
    entities: Query<Entity, Without<Parent>>
) {
    for (character, super::UnloadedCharacter(scene), parent) in query.iter() {
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(*scene) {
            entity_iter.for_each(|entity| {
                if entities.get(entity).is_ok() {
                    let mut char_transform = Transform::identity();
                    char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
                    commands.entity(entity).insert_bundle(super::CharacterBundle::from(char_transform));
                    commands.entity(parent.0).push_children(&[entity]);
                } else {
                    commands.entity(entity).insert(super::CharacterEntity);
                }
            });
            commands.entity(character).despawn_recursive();
        }
    }
}
