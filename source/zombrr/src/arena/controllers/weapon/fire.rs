use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::*;

pub fn handle_fire_weapon(
    mut events: EventReader<FireWeapon>,
    query_pipeline: Res<QueryPipeline>,
    colliders: QueryPipelineColliderComponentsQuery,
    weapon_entities: Query<(&Name, &GlobalTransform), With<WeaponEntity>>,
    mut weapons: Query<(&mut Magazine, &Children), With<WeaponRoot>>
) {
    for FireWeapon(entity) in events.iter() {
        if let Ok((mut magazine, children)) = weapons.get_mut(*entity) {
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
                    let ray = Ray::new(
                        eyepoint.unwrap().translation.into(),
                        shotpoint.unwrap().translation.into()
                    );
                    let colliders = QueryPipelineColliderComponentsSet(&colliders);
                    if let Some((handle, intersection)) = query_pipeline.cast_ray(
                         &colliders, &ray, 4.0, true, InteractionGroups::all(), None
                    ) {
                        println!("{:?} - {:?}", intersection, handle);
                    }
                }
            }
        }
    }
}
