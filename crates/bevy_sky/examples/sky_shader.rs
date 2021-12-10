use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_devtools::{DevToolsExt, DevToolsPlugin};
use bevy_sky::{SkyBundle, SkyCameraTag, SkyMaterial, SkyPlugin};

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
