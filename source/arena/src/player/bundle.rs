use bevy::prelude::*;
use bevy_hilt::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::controllers::navigatable::{
    Navigatable, KeyboardInput, MouseInput
};
use crate::controllers::damage::Damage;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub health: Damage,
    pub root: super::PlayerRoot,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub navigatable: Navigatable,
    pub keyboard: KeyboardInput,
    pub mouse: MouseInput,
    #[bundle]
    pub rigid_body: RigidBodyBundle,
    #[bundle]
    pub collider: ColliderBundle,
    pub position_sync: RigidBodyPositionSync,
    pub debug_collider: HiltDebugCollider,
    pub debug_position: HiltDebugPosition
}

impl From<Transform> for PlayerBundle {
    fn from(transform: Transform) -> PlayerBundle {
        PlayerBundle {
            name: Name::new("Player"),
            root: super::PlayerRoot,
            transform,
            health: Damage::default(),
            global_transform: GlobalTransform::identity(),
            navigatable: Navigatable::default(),
            keyboard: KeyboardInput,
            mouse: MouseInput { sensitivity: 10.0, disabled: false },
            rigid_body: RigidBodyBundle {
                body_type: RigidBodyType::Dynamic,
                position: (
                    transform.translation,
                    transform.rotation
                ).into(),
                mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                ..Default::default()
            },
            collider: ColliderBundle {
                shape: ColliderShape::cuboid(0.25, 1.0, 0.25),
                position: Vec3::new(0.0, 1.0, 0.0).into(),
                mass_properties: ColliderMassProps::Density(1.0),
                flags: ActiveEvents::CONTACT_EVENTS.into(),
                ..Default::default()
            },
            position_sync: RigidBodyPositionSync::Discrete,
            debug_collider: HiltDebugCollider { color: Color::TEAL },
            debug_position: Default::default()
        }
    }
}
