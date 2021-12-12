use bevy::prelude::*;
use bevy_hilt::prelude::*;
use bevy_rapier3d::prelude::*;

use zombrr_health::Health;
use crate::controllers::navigatable::Navigatable;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub name: Name,
    pub health: Health,
    pub root: zombrr_core::EnemyRoot,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub navigatable: Navigatable,
    #[bundle]
    pub rigid_body: RigidBodyBundle,
    #[bundle]
    pub collider: ColliderBundle,
    pub position_sync: RigidBodyPositionSync,
    pub debug_collider: HiltDebugCollider,
    pub debug_position: HiltDebugPosition,
}

impl EnemyBundle {
    pub fn new(transform: Transform, speed: f32) -> EnemyBundle {
        EnemyBundle {
            name: Name::new("Enemy"),
            health: Health::default(),
            root: zombrr_core::EnemyRoot,
            transform,
            global_transform: GlobalTransform::identity(),
            navigatable: Navigatable {
                speed,
                ..Default::default()
            },
            rigid_body: RigidBodyBundle {
                body_type: RigidBodyType::Dynamic,
                position: (transform.translation, transform.rotation).into(),
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
            debug_collider: HiltDebugCollider { color: Color::RED },
            debug_position: Default::default(),
        }
    }
}
