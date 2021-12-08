use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::*;
use zombrr_core::packages::WeaponMeta;
use crate::arena::controllers::damage::SustainedDamage;

pub fn handle_fire_weapon(
    mut events: EventReader<FireWeapon>,
    query_pipeline: Res<QueryPipeline>,
    colliders: QueryPipelineColliderComponentsQuery,
    mut health_events: EventWriter<SustainedDamage>,
    weapon_entities: Query<(&Name, &GlobalTransform), With<WeaponEntity>>,
    mut weapons: Query<(&mut Magazine, &Children, &WeaponMeta), With<WeaponRoot>>
) {
    for FireWeapon { weapon, assailant } in events.iter() {
        if let Ok((mut magazine, children, weapon_meta)) = weapons.get_mut(*weapon) {
            debug!("Firing Weapon: Entity({:?})", *weapon);
            if magazine.fire() {
                let mut eyepoint = None;
                let mut shotpoint = None;
                for child in children.iter() {
                    if let Ok((name, transform)) = weapon_entities.get(*child) {
                        if name.as_str() == "Weapon:ShotPoint" {
                            shotpoint = Some(*transform);
                        }
                        if name.as_str() == "Weapon:EyePoint" {
                            eyepoint = Some(*transform)
                        }
                    }
                }
                if eyepoint.is_some() && shotpoint.is_some() {
                    let eye = eyepoint.unwrap();
                    let shot = shotpoint.unwrap();
                    let ray = Ray::new(
                        shot.translation.into(),
                        (Vec3::from(shot.translation - eye.translation)).into()
                    );
                    let colliders = QueryPipelineColliderComponentsSet(&colliders);
                    if let Some((handle, _)) = query_pipeline.cast_ray(
                         &colliders, &ray, 1000.0, true, InteractionGroups::all(), None
                    ) {
                        health_events.send(SustainedDamage {
                            value: weapon_meta.damage,
                            target: handle.entity(),
                            assailant: *assailant
                        });
                    }
                }
            }
        }
    }
}
