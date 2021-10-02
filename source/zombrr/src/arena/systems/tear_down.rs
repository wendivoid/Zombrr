use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn tear_down_physics(
    mut rapier: ResMut<RapierConfiguration>
) {
    rapier.physics_pipeline_active = false;
    rapier.query_pipeline_active = false;
}
