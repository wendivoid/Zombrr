use bevy::prelude::*;

pub mod navigatable;

pub struct ArenaControllersPlugin;

impl Plugin for ArenaControllersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(navigatable::NavigatablePlugin);
    }
}
