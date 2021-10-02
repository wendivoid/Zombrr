use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Navigatable;

pub fn navigatable_move(
    mut query: Query<(&Navigatable, &mut RigidBodyPosition, &mut RigidBodyVelocity), Changed<Navigatable>>
) {
    for (navigatable, mut position, mut velocity) in query.iter_mut() {
        let mut vel: Vec3 = velocity.linvel.into();
        vel += navigatable.velocity;
        velocity.linvel = vel.into();
        position.position.rotation = navigatable.get_look_quat().into();
    }
}
