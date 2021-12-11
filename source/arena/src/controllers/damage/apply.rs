use bevy::prelude::*;
use std::time::Duration;
use bevy_rapier3d::prelude::*;

use super::*;

pub struct BulletDecalTimer(Timer);

pub fn apply_health(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut death_events: EventWriter<Death>,
    mut events: EventReader<SustainedDamage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut health_entities: Query<(Option<&mut Damage>, Option<&zombrr_core::BulletHoles>), With<ColliderShape>>,
) {
    for SustainedDamage {
        target,
        value,
        assailant,
        point,
        surface_normal
    } in events.iter()
    {
        if let Ok((health, bullet)) = health_entities.get_mut(*target) {
            debug!("Dealing `{}` damge to: Entity({:?})", value, *target);
            if let Some(mut health) = health {
                if health.apply(*value) {
                    death_events.send(Death {
                        target: *target,
                        assailant: *assailant,
                    });
                }
            }
            if let Some(bullet) = bullet {
                let mut transform = Transform::from_translation(*point);
                transform.scale = Vec3::new(0.001, 0.001, 1.0);
                transform.look_at(*point + *surface_normal, Vec3::Y);

                let mut builder = commands.spawn_bundle(SpriteBundle {
                    transform,
                    material: materials.add(assets.load("packages/zombrr/zombrr/textures/bullet_hole.png").into()),
                    ..Default::default()
                });
                if let Some(delay) = bullet.disappear_after {
                    builder.insert(BulletDecalTimer(Timer::new(Duration::from_secs(delay), false)));
                }
            }
        }
    }
}

pub fn remove_bullet_decals(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut BulletDecalTimer)>
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
