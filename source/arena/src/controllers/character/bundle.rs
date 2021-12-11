use bevy::prelude::*;

#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub character: zombrr_core::CharacterRoot,
}

impl From<Transform> for CharacterBundle {
    fn from(transform: Transform) -> CharacterBundle {
        CharacterBundle {
            transform,
            name: Name::new("Character"),
            global_transform: GlobalTransform::identity(),
            character: zombrr_core::CharacterRoot,
        }
    }
}
