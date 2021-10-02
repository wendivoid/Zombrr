use bevy::prelude::*;

pub fn track_camera(
    transforms: QuerySet<(
        Query<&GlobalTransform, (With<crate::SkyCameraTag>, Changed<GlobalTransform>)>,
        Query<&mut GlobalTransform, With<Handle<crate::SkyMaterial>>>,
    )>,
) {
    let mut cam_temp = transforms.q0().iter();
    if let Some(camera_transform) = cam_temp.next() {
        transforms
            .q1()
            .for_each_mut(|mut mesh_transform| *mesh_transform = *camera_transform);
    }
}
