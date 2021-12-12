use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod ray;

use super::*;
use zombrr_health::Damage;
use zombrr_core::packages::{WeaponMeta, WeaponAction};
use zombrr_core::{WeaponRoot, WeaponEntity};

pub fn handle_fire_weapon(
    mut commands: Commands,
    mut events: EventReader<FireWeapon>,
    query_pipeline: Res<QueryPipeline>,
    colliders: QueryPipelineColliderComponentsQuery,
    health_events: EventWriter<Damage>,
    weapon_entities: Query<(&Name, &GlobalTransform), With<WeaponEntity>>,
    mut weapons: Query<(&mut Magazine, &Children, &WeaponMeta), With<WeaponRoot>>,
) {
    if let Some(FireWeapon { weapon, assailant }) = events.iter().next() {
        if let Ok((mut magazine, children, weapon_meta)) = weapons.get_mut(*weapon) {
            debug!("Firing Weapon: Entity({:?})", *weapon);
            if magazine.fire() {
                match &weapon_meta.action {
                    WeaponAction::Ray { range, tracer_color } => {
                        ray::fire_ray(
                            &mut commands,
                            *range,
                            crate::utils::zombrr_color_to_bevy(tracer_color),
                            children,
                            *assailant,
                            weapon_meta,
                            query_pipeline,
                            colliders,
                            health_events,
                            weapon_entities
                        );
                    }
                }
            }
        }
    }
}
