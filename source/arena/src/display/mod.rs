mod init;

mod plugin;
pub use self::plugin::DisplayPlugin;

use bevy::prelude::*;

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct FpsText;

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct KillCountText;

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct DisplayRoot;

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct TextField;

pub fn add_focus_policy(mut commands: Commands, query: Query<Entity, Changed<TextField>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .remove::<TextField>()
            .insert(bevy::ui::FocusPolicy::default())
            .insert(bevy::ui::CalculatedSize::default());
    }
}
