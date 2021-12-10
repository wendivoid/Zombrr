use bevy::app::Events;
use bevy::prelude::*;
use bevy_devtools::egui::{self, Ui};
use bevy_devtools::{Setting, SettingValue, Settings, Tool};

use zombrr_arena::SpawnEnemy;

pub fn settings() -> Setting {
    Setting::new_labeled("enemy", "Enemy").set_value(SettingValue::Group(vec![
        Setting::new_labeled("default_pos_x", "Default Position X").set_value_float(0.0),
        Setting::new_labeled("default_pos_y", "Default Position Y").set_value_float(5.0),
        Setting::new_labeled("default_pos_z", "Default Position Z").set_value_float(0.0),
        Setting::new_labeled("speed", "Default Speed").set_value_float(0.2),
    ]))
}

pub fn spawn_enemy() -> Tool {
    Tool {
        name: "spawn-enemy".into(),
        label: Some("Spawn Enemy".into()),
        priority: 0,
        perform_icon: Some("Spawn".into()),
        perform: Some(perform_spawn_enemy),
        render: render_spawn_enemy,
    }
}
fn perform_spawn_enemy(world: &mut World) {
    let spawn_event = {
        let settings = world.get_resource::<Settings>().unwrap();
        let pos_x = settings
            .get_key(&["enemy", "default_pos_x"])
            .unwrap()
            .value
            .as_float()
            .unwrap();
        let pos_y = settings
            .get_key(&["enemy", "default_pos_y"])
            .unwrap()
            .value
            .as_float()
            .unwrap();
        let pos_z = settings
            .get_key(&["enemy", "default_pos_z"])
            .unwrap()
            .value
            .as_float()
            .unwrap();
        let speed = settings
            .get_key(&["enemy", "speed"])
            .unwrap()
            .value
            .as_float()
            .unwrap();
        SpawnEnemy {
            translation: Vec3::new(pos_x, pos_y, pos_z),
            character: Default::default(),
            speed,
        }
    };
    let mut events = world.get_resource_mut::<Events<SpawnEnemy>>().unwrap();
    events.send(spawn_event);
}

fn render_spawn_enemy(ui: &mut Ui, settings: &mut Settings, _: &mut World) {
    ui.horizontal(|ui| {
        ui.set_max_width(150.0);
        ui.columns(3, |ui| {
            {
                let pos_x = settings
                    .get_key_mut(&["enemy", "default_pos_x"])
                    .unwrap()
                    .value
                    .as_float_mut()
                    .unwrap();
                ui[0].add(egui::DragValue::new(pos_x));
            }
            {
                let pos_y = settings
                    .get_key_mut(&["enemy", "default_pos_y"])
                    .unwrap()
                    .value
                    .as_float_mut()
                    .unwrap();
                ui[1].add(egui::DragValue::new(pos_y));
            }
            {
                let pos_z = settings
                    .get_key_mut(&["enemy", "default_pos_z"])
                    .unwrap()
                    .value
                    .as_float_mut()
                    .unwrap();
                ui[2].add(egui::DragValue::new(pos_z));
            }
        });
        ui.label("Location");
    });
    ui.end_row();
    ui.horizontal(|ui| {
        ui.columns(2, |ui| {
            let speed = settings
                .get_key_mut(&["enemy", "speed"])
                .unwrap()
                .value
                .as_float_mut()
                .unwrap();
            ui[0].add(egui::DragValue::new(speed));
            ui[1].label("Speed");
        });
    });
}
