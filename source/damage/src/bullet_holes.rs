use bevy::prelude::*;
use zombrr_health::*;
use std::time::Duration;

pub struct BulletHoles {
    pub disappear_after: Option<u64>
}

impl Default for BulletHoles {
    fn default() -> BulletHoles {
        BulletHoles {
            disappear_after: Some(5)
        }
    }
}

pub struct BulletDecalTimer(Timer);

pub fn draw_bullet_holes(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut events: EventReader<Damage>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut health_entities: Query<&BulletHoles>
) {
    for Damage {
        target,
        point,
        surface_normal,
        ..
    } in events.iter() {
        if let Ok(bullet) = health_entities.get_mut(*target) {

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

pub fn remove_bullet_holes(
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
