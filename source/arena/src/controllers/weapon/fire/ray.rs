use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use zombrr_health::Damage;
use zombrr_core::packages::WeaponMeta;
use zombrr_core::WeaponEntity;

pub fn fire_ray(
    commands: &mut Commands,
    range: f32,
    tracer_color: Color,
    children: &Children,
    assailant: Entity,
    weapon_meta: &WeaponMeta,
    query_pipeline: Res<QueryPipeline>,
    colliders: QueryPipelineColliderComponentsQuery,
    mut health_events: EventWriter<Damage>,
    weapon_entities: Query<(&Name, &GlobalTransform), With<WeaponEntity>>,
) {
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
                (Vec3::from(shot.translation - eye.translation)).into(),
            );
            let colliders = QueryPipelineColliderComponentsSet(&colliders);
            if let Some((handle, normal)) = query_pipeline.cast_ray_and_get_normal(
                &colliders,
                &ray,
                range,
                true,
                InteractionGroups::all(),
                None,
            ) {
                let point = ray.point_at(normal.toi - 0.01);
                commands
                .spawn()
                .insert(crate::controllers::tracers::Tracer {
                    length: 150,
                    color: tracer_color,
                    point: shot.translation,
                    target: point.into(),
                });
                health_events.send(Damage {
                    value: weapon_meta.damage,
                    target: handle.entity(),
                    assailant,
                    point: point.into(),
                    surface_normal: normal.normal.into()
                });
            }
        }
}
