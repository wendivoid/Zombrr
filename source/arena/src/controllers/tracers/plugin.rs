use bevy::prelude::*;

pub struct TracersPlugin;

impl Plugin for TracersPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(super::setup::setup_tracer_graphics.system())
            .add_system(super::trace::spawn_trace_bundles.system())
            .add_system(super::animate::animate_tracers.system());
    }
}
