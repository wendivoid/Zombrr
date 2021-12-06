use bevy::prelude::*;

#[derive(Bundle)]
pub struct WeaponBundle {
    pub name: Name,
    pub transform: Transform,
    pub global_transform: GlobalTransform
}

impl Default for WeaponBundle {
    fn default() -> WeaponBundle {
        WeaponBundle {
            name: Name::new("Weapon"),
            transform: Default::default(),
            global_transform: Default::default()
        }
    }
}

impl WeaponBundle {

    pub fn spawn(parent: &mut ChildBuilder, transform: Transform, scene_handle: Handle<Scene>) {
        parent.spawn_bundle(WeaponBundle {
            transform,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_scene(scene_handle);
        });

    }
}
