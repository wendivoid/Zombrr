use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_physics(mut rapier: ResMut<RapierConfiguration>) {
    rapier.physics_pipeline_active = true;
    rapier.query_pipeline_active = true;
}
