use bevy::prelude::*;
use bevy_devtools::{Setting, SettingValue, Settings};

pub fn settings() -> Setting {
    Setting::new_labeled("rapier-hilt", "Rapier Hilt")
        .set_value(SettingValue::Group(vec![
            Setting::new_labeled("show-colliders", "Show Colliders").set_value_bool(true),
            Setting::new_labeled("render-pass", "Show in Hilt Pass")
        ]))
}

pub fn show_colliders_changed(
    mut last_value: Local<bool>,
    settings: Res<Settings>,
    mut events: EventWriter<bevy_hilt::HiltToggleVisibility>
) {
  if let Some(setting) = settings.get_key(&["rapier-hilt", "show-colliders"]) {
      if let Some(value) = setting.value.as_bool() {
          if *last_value != value {
              *last_value = value;
              events.send(bevy_hilt::HiltToggleVisibility(bevy_hilt::HiltEntities::All));
          }
      } else {
          error!("DevTools Settings key `rapier-hilt -> show-colliders` is not a bool");
      }
  } else {
      error!("Cant find DevTools Settings key `rapier-hilt -> show-colliders`");
  }
}

pub fn show_render_pass_changed(
    mut last_value: Local<bool>,
    settings: Res<Settings>,
    mut events: EventWriter<bevy_hilt::HiltToggleRenderPass>
) {
  if let Some(setting) = settings.get_key(&["rapier-hilt", "render-pass"]) {
      if let Some(value) = setting.value.as_bool() {
          if *last_value != value {
              *last_value = value;
              events.send(bevy_hilt::HiltToggleRenderPass(bevy_hilt::HiltEntities::All));
          }
      } else {
          error!("DevTools Settings key `rapier-hilt -> render-pass` is not a bool");
      }
  } else {
      error!("Cant find DevTools Settings key `rapier-hilt -> render-pass`");
  }
}
