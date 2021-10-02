use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<crate::SkyMaterial>()
            .add_startup_system_to_stage(StartupStage::PreStartup, crate::systems::setup.system())
            .add_system(crate::systems::pass_time.system())
            .add_system_to_stage(
                CoreStage::PostUpdate,
                crate::systems::track_camera
                    .system()
                    .after(TransformSystem::ParentUpdate),
            );
    }
}
