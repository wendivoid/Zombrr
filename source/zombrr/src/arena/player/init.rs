use bevy::prelude::*;
use bevy_loading::Progress;
use bevy_rapier3d::prelude::*;
use bevy_hilt::prelude::{HiltDebugCollider, HiltDebugPosition};

pub fn init_player(
    mut commands: Commands,
    query: Query<(&Name, &GlobalTransform), With<crate::arena::map::ArenaGltfMapObject>>
) -> Progress {
    for (name, global_transform) in query.iter() {
        if name.as_str() == "PlayerSpawn" {
            commands.spawn()
                .insert(Name::new("Player"))
                .insert(super::PlayerRoot)
                .insert(*global_transform)
                .insert(Transform::from(*global_transform))
                .insert(crate::arena::controllers::navigatable::Navigatable::default())
                .insert(crate::arena::controllers::navigatable::KeyboardInput)
                .insert(crate::arena::controllers::navigatable::MouseInput { sensitivity: 10.0 })
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
                    shape: ColliderShape::capsule(
                        Vec3::new(0.0, 0.0, 0.0).into(),
                        Vec3::new(0.0, 4.0, 0.0).into(),
                        1.0
                    ),
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
                        .insert(Transform::from_xyz(0.0, 2.0, 0.0))
                        .insert(GlobalTransform::from_xyz(0.0, 2.0, 0.0))
                        .with_children(|parent| {
                            parent.spawn_bundle(PerspectiveCameraBundle::new_3d())
                                .insert(bevy_sky::SkyCameraTag);
                            parent.spawn_bundle(bevy_hilt::prelude::HiltPerspectiveCameraBundle::default());
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
