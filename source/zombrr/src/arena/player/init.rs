use bevy::prelude::*;
use bevy_loading::Progress;
use bevy_rapier3d::prelude::*;
use bevy_hilt::prelude::{HiltDebugCollider, HiltDebugPosition};
use zombrr_core::{ ZombrrPackages, ArenaOptions };

pub fn init_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    options: Res<ArenaOptions>,
    query: Query<(&Name, &GlobalTransform), With<crate::arena::map::ArenaGltfMapObject>>
) -> Progress {
    for (name, global_transform) in query.iter() {
        if name.as_str() == "PlayerSpawn" {
            let character = packages.get_character(&options.player.character).unwrap();
            let weapon = packages.get_weapon(&options.player.weapon).unwrap();
            commands.spawn()
                .insert(Name::new("Player"))
                .insert(super::PlayerRoot)
                .insert(*global_transform)
                .insert(Transform::from(*global_transform))
                .insert(crate::arena::controllers::navigatable::Navigatable::default())
                .insert(crate::arena::controllers::navigatable::KeyboardInput)
                .insert(crate::arena::controllers::navigatable::MouseInput {
                    sensitivity: 10.0, disabled: false
                })
                .insert_bundle(RigidBodyBundle {
                    body_type: RigidBodyType::Dynamic,
                    position: (
                        global_transform.translation,
                        global_transform.rotation
                    ).into(),
                    mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                    ..Default::default()
                })
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(0.25, 1.0, 0.25),
                    position: Vec3::new(0.0, 1.0, 0.0).into(),
                    mass_properties: ColliderMassProps::Density(1.0),
                    flags: ActiveEvents::CONTACT_EVENTS.into(),
                    ..Default::default()
                })
                .insert(HiltDebugCollider {
                    color: Color::TEAL
                })
                .insert(RigidBodyPositionSync::Discrete)
                .insert(HiltDebugPosition::default())
                .with_children(|parent| {
                    parent.spawn()
                        .insert(Name::new("Cameras"))
                        .insert(Transform::from_xyz(0.0, 2.0, 0.7))
                        .insert(GlobalTransform::from_xyz(0.0, 2.0, 0.0))
                        .with_children(|parent| {
                            parent.spawn_bundle(PerspectiveCameraBundle::new_3d())
                                .insert(bevy_sky::SkyCameraTag);
                            parent.spawn_bundle(bevy_hilt::prelude::HiltPerspectiveCameraBundle::default());
                        });
                    let mut char_transform = Transform::identity();
                    char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
                    parent.spawn()
                        .insert(Name::new("Character"))
                        .insert(char_transform)
                        .insert(GlobalTransform::identity())
                        .with_children(|parent| {
                            parent.spawn_scene(assets.load(character.scene_file().as_str()));
                        });
                    parent.spawn()
                        .insert(Name::new("Weapon"))
                        .insert(Transform::from_xyz(0.3, 1.4, -1.0))
                        .insert(GlobalTransform::identity())
                        .with_children(|parent| {
                            parent.spawn_scene(assets.load(weapon.scene_file().as_str()));
                        });
                });
            return Progress { done: 1, total: 1 };
        }
    }
    Progress {
        done: 0,
        total: 1
    }
}
