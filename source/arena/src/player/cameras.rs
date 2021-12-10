use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerCamerasBundle {
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for PlayerCamerasBundle {
    fn default() -> PlayerCamerasBundle {
        PlayerCamerasBundle {
            name: Name::new("Camaras"),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}

impl PlayerCamerasBundle {
    pub fn spawn(parent: &mut ChildBuilder, transform: Transform) {
        parent
            .spawn_bundle(PlayerCamerasBundle {
                transform,
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(PerspectiveCameraBundle::new_3d())
                    .insert(bevy_sky::SkyCameraTag);
                parent.spawn_bundle(bevy_hilt::prelude::HiltPerspectiveCameraBundle::default());
            });
    }
}
