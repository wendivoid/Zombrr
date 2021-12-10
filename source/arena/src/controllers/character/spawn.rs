use bevy::prelude::*;
use zombrr_core::ZombrrPackages;

pub fn handle_character_spawns(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut events: EventReader<super::SpawnCharacter>,
) {
    for super::SpawnCharacter { parent, character } in events.iter() {
        let character = packages.get_character(character).unwrap();
        debug!(
            "Starting Character Spawn Process: `{}`",
            character.text_label()
        );
        let scene_handle = assets.load(character.scene_file().as_str());
        let instance_id = scene_spawner.spawn(scene_handle);

        let mut char_transform = Transform::identity();
        char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));

        commands
            .entity(*parent)
            .insert(char_transform)
            .insert(GlobalTransform::identity())
            .with_children(|parent| {
                parent.spawn().insert(super::UnloadedCharacter(instance_id));
            });
    }
}
