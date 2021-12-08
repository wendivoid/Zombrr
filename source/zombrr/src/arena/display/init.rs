use bevy::prelude::*;
use bevy_loading::*;
use bevy::asset::LoadState;
use zombrr_core::{ArenaOptions, ZombrrPackages};

pub fn initialize_user_interface(
    mut commands: Commands,
    assets: Res<AssetServer>,
    options: Res<ArenaOptions>,
    packages: Res<ZombrrPackages>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(environment) = packages.get_display(&options.player.display) {
        let mut path = environment.path.clone();
        path.push(&environment.meta.scene);
        debug!("Loading Environment Scene: {:?}", path);
        scene_spawner.spawn_dynamic(assets.load(path.to_str().unwrap()));
        commands.spawn_bundle(UiCameraBundle::default());
        commands.spawn_bundle((
            crate::arena::UserInterfaceRoot,
            Name::new("User Interface"),
        )).insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            ..Default::default()
        });
    }
}

pub fn check_environment_scene(
    mut commands: Commands,
    assets: Res<AssetServer>,
    options: Res<ArenaOptions>,
    packages: Res<ZombrrPackages>,
    interfaces: Query<Entity, With<crate::arena::interface::UserInterfaceRoot>>,
    entities: Query<Entity, (With<super::DisplayRoot>, Without<Parent>)>
) -> Progress {

    if let Some(environment) = packages.get_display(&options.player.display) {
        let mut path = environment.path.clone();
        path.push(&environment.meta.scene);
        if let LoadState::Loaded = assets.get_load_state(path.to_str().unwrap()) {
            if let Ok(entity) = entities.single() {
                let interface = interfaces.single().unwrap();
                commands.entity(interface).push_children(&[entity]);
            }
            return Progress { done: 1, total: 1 };
        }
    }

    Progress {
        done: 0,
        total: 1
    }
}
