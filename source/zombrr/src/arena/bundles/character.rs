use bevy::prelude::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform
}

impl Default for CharacterBundle {
    fn default() -> CharacterBundle {
        CharacterBundle {
            name: Name::new("Character"),
            transform: Default::default(),
            global_transform: Default::default()
        }
    }
}

impl CharacterBundle {

    pub fn spawn(parent: &mut ChildBuilder, transform: Transform, scene_handle: Handle<Scene>) {
        parent.spawn_bundle(CharacterBundle {
            transform,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_scene(scene_handle);
        });

    }
}
