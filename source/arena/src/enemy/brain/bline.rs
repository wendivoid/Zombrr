use bevy::prelude::*;

use crate::controllers::navigatable::Navigatable;

pub struct BLine {
    pub to: Entity,
}

pub fn handle_bline(
    transforms: Query<&Transform>,
    mut query: Query<(Entity, &mut Navigatable, &BLine)>,
) {
    for (entity, mut navigatable, bline) in query.iter_mut() {
        navigatable.velocity = Vec3::ZERO;
        let follow = transforms.get(bline.to).unwrap();
        let mut current = transforms.get(entity).unwrap().clone();
        current.look_at(follow.translation, Vec3::Y);

        navigatable.yaw = current.rotation.to_axis_angle().1;

        let forward =
            (navigatable.get_look_quat() * Vec3::new(0.0, 0.0, -1.0) * Vec3::new(1.0, 0.0, 1.0))
                .normalize()
                .clamp_length_max(navigatable.speed);
        navigatable.velocity += forward;
    }
}
