use bevy::prelude::*;

pub fn handle_fire_input(
    mouse: Res<Input<MouseButton>>,
    mut events: EventWriter<super::FireWeapon>,
    weapon: Query<Entity, With<super::WeaponRoot>>
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(weapon) = weapon.single() {
            events.send(super::FireWeapon(weapon));
        }
    }
}
