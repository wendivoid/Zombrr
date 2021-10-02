use bevy::{
    prelude::*,
    input::system::exit_on_esc_system,
};
use bevy_sky::{ SkyMaterial, SkyCameraTag, SkyBundle, SkyPlugin, Sun };
use bevy_devtools::{DevToolsPlugin, DevToolsExt};

use chrono::{ Utc, TimeZone };

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_system(exit_on_esc_system.system())
        .add_plugin(SkyPlugin)
        .add_plugin(DevToolsPlugin::<()>::default())
        .devtools_enabled()
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut sky_materials: ResMut<Assets<SkyMaterial>>,
) {
        commands.spawn_bundle(SkyBundle {
            sun: Sun {
                // Stockholm
                latitude: 59.33258,
                longitude: 18.0649,
                // one day per 30 seconds of real time
                simulation_seconds_per_second: 24.0 * 60.0 * 60.0 / 10.0,
                now: Utc.ymd(2021, 03, 01).and_hms(7, 0, 0),
                active: true,
                ..Default::default()
            },
            mesh: meshes.add(Mesh::from(shape::Cube { size: 15.0 })),
            material: sky_materials.add(SkyMaterial::default()),
            ..Default::default()
        });

        // camera
        commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(SkyCameraTag);
}
