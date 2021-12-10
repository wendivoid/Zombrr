use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn navigatable_collision(
    mut contact_events: EventReader<ContactEvent>,
    colliders: Query<&ColliderPosition, With<ColliderShape>>,
    mut navigatables: Query<(&mut super::Navigatable, &ColliderPosition)>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                if let Ok((mut navigatable, pos)) = navigatables.get_mut(h1.entity()) {
                    if let Ok(c_pos) = colliders.get(h2.entity()) {
                        if pos.translation.y > c_pos.translation.y {
                            navigatable.on_solid = true;
                        }
                    }
                } else if let Ok((mut navigatable, pos)) = navigatables.get_mut(h2.entity()) {
                    if let Ok(c_pos) = colliders.get(h1.entity()) {
                        if pos.translation.y > c_pos.translation.y {
                            navigatable.on_solid = true;
                        }
                    }
                }
            }
            ContactEvent::Stopped(h1, h2) => {
                if let Ok((mut navigatable, pos)) = navigatables.get_mut(h1.entity()) {
                    if let Ok(c_pos) = colliders.get(h2.entity()) {
                        if pos.translation.y > c_pos.translation.y {
                            navigatable.on_solid = false;
                        }
                    }
                } else if let Ok((mut navigatable, pos)) = navigatables.get_mut(h2.entity()) {
                    if let Ok(c_pos) = colliders.get(h1.entity()) {
                        if pos.translation.y > c_pos.translation.y {
                            navigatable.on_solid = false;
                        }
                    }
                }
            }
        }
    }
}
