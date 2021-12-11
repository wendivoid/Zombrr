use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn tear_down_physics(
    mut rapier: ResMut<RapierConfiguration>,
) {
    rapier.physics_pipeline_active = false;
    rapier.query_pipeline_active = false;
}

pub fn tear_down_user_interface(
    mut commands: Commands,
    ui: Query<Entity, With<crate::UserInterfaceRoot>>,
    ui_camera: Query<Entity, With<crate::interface::UserInterfaceCamera>>
) {
    commands.entity(ui.single().unwrap()).despawn_recursive();
    commands.entity(ui_camera.single().unwrap()).despawn();
}

pub fn tear_down_enemies(
    mut commands: Commands,
    entities: Query<Entity, With<crate::enemy::EnemyRoot>>
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
