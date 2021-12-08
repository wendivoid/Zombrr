use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<super::SpawnCharacter>()
            .add_system(super::spawn::handle_character_spawns.system())
            .add_system(super::finalize::finalize_character_spawns.system());
    }
}
