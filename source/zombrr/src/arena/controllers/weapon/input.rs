use bevy::prelude::*;

pub fn handle_fire_input(
    mouse: Res<Input<MouseButton>>,
    mut events: EventWriter<super::FireWeapon>,
    weapon: Query<Entity, With<super::WeaponRoot>>
) {
    if mouse.just_pressed(MouseButton::Left) {
        let weapon = weapon.single().unwrap();
        events.send(super::FireWeapon(weapon));
    }
}
