use bevy::prelude::*;
use bevy_hilt::prelude::*;
use bevy_rapier3d::prelude::*;
use zombrr_core::ZombrrPackages;

use crate::arena::SpawnEnemy;

pub fn spawn_enemy(
    mut commands: Commands,
    assets: Res<AssetServer>,
    packages: Res<ZombrrPackages>,
    players: Query<Entity, With<crate::arena::player::PlayerRoot>>,
    mut events: EventReader<SpawnEnemy>,
) {
    for SpawnEnemy { translation, character } in events.iter() {
        let player = players.single().unwrap();
        debug!("Spawning Enemy with character: {:?}", character);
        let character = packages.get_character(character).unwrap();
        let mut character_path = character.path.clone();
        character_path.push(&character.meta.scene);
        let asset_path = format!("{}#Scene0", character_path.to_str().unwrap());
        let mut char_transform = Transform::identity();
        char_transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
        commands.spawn()
            .insert(Name::new("Enemy"))
            .insert(Transform::identity())
            .insert(GlobalTransform::identity())
            .insert(super::brain::BLine { to: player })
            .insert(crate::arena::controllers::navigatable::Navigatable {
                speed: 0.2,
                ..Default::default()
            })
            .insert(super::EnemyRoot)
            .insert_bundle(RigidBodyBundle {
                body_type: RigidBodyType::Dynamic,
                position: (*translation).into(),
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
                color: Color::RED
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(HiltDebugPosition::default())
            .with_children(|parent| {
                parent.spawn()
                    .insert(Name::new("Character"))
                    .insert(char_transform)
                    .insert(GlobalTransform::default())
                    .with_children(|parent| {
                        parent.spawn_scene(assets.load(asset_path.as_str()));
                    });
            });
    }
}