use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_prototype_debug_lines::DebugLines;
use bevy_devtools::{Setting, SettingValue, Settings};
use crate::arena::controllers::weapon::WeaponEntity;

pub fn settings() -> Setting {
    Setting::new_labeled("weapons", "Weapons")
        .set_value(SettingValue::Group(vec![
            Setting::new_labeled("shot-rays", "Show Shot Rays").set_value_bool(true)
        ]))
}

pub fn display_shot_rays(
    settings: Res<Settings>,
    mut lines: ResMut<DebugLines>,
    query: Query<(&Name, &GlobalTransform, &Parent), With<WeaponEntity>>
) {
    if settings.get_key(&["weapons", "shot-rays"]).unwrap().value.as_bool().unwrap() {
        let mut weapon_map: HashMap<Entity, (GlobalTransform, GlobalTransform)> = HashMap::default();
        for (name, transform, parent) in query.iter() {

            if name.as_str() == "Weapon:ShotPoint" {
                if weapon_map.contains_key(&parent.0) {
                    let entity = weapon_map.get_mut(&parent.0).unwrap();
                    *entity = (entity.0, *transform);
                } else {
                    weapon_map.insert(parent.0, (GlobalTransform::identity(), *transform));
                }
            }

            if name.as_str() == "Weapon:EyePoint" {
                if weapon_map.contains_key(&parent.0) {
                    let parent = weapon_map.get_mut(&parent.0).unwrap();
                    *parent = (*transform, parent.1);
                } else {
                    weapon_map.insert(parent.0, (*transform, GlobalTransform::identity()));
                }
            }
        }
        for (_, (eye_point, shot_point)) in weapon_map.iter() {
            lines.line_colored(eye_point.translation, shot_point.translation, 0.0, Color::MAROON);
        }
    }


}
