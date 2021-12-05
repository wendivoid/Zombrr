use bevy::prelude::*;
use bevy::app::Events;
use bevy_devtools::{
    Panel,
    egui::{self, Ui},
    bevy_egui::EguiContext
};
use zombrr_core::{ZombrrPackages, ArenaOptions};

pub fn change_arena_map() -> Panel {
    Panel::new("☢").render(draw_zombrr_panel)
}

fn draw_zombrr_panel(_: &EguiContext, ui: &mut Ui, world: &mut World) {
    let ((map, original_map), (spawn_enemy, enemy)) = {
        let packages = world.get_resource::<ZombrrPackages>().unwrap();
        let options = world.get_resource::<ArenaOptions>().unwrap();
        let original_map = (options.map.package.as_str(), options.map.package.as_str(), options.map.name.as_str());
        let mut selected_map = (options.map.package.as_str(), options.map.package.as_str(), options.map.name.as_str());
        ui.group(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Change Map")
                .selected_text(&options.map.name)
                .show_ui(ui, |ui| {
                    for (namespace, packages) in packages.get_maps() {
                        for (package, maps) in packages {
                            for map in maps {
                                ui.selectable_value(&mut selected_map, (namespace, package, &map.name), &map.name);
                            }
                        }
                    }
                });
            });
        });
        let mut spawn = false;
        let mut selected_character = (options.enemy.character.namespace.as_str(), options.enemy.character.package.as_str(), options.enemy.character.name.as_str());
        ui.group(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Character")
                    .selected_text(&options.enemy.character.name)
                    .show_ui(ui, |ui| {
                        for (namespace, packages) in packages.get_characters() {
                            for (package, characters) in packages {
                                for character in characters {
                                    ui.selectable_value(&mut selected_character, (namespace, package, &character.name), &character.name);
                                }
                            }
                        }
                    });
                spawn = ui.button("Spawn").clicked();
            });
        });
        (
            (
                (selected_map.0.to_string(), selected_map.1.to_string(), selected_map.2.to_string()),
                (original_map.0.to_string(), original_map.1.to_string(), selected_map.2.to_string())
            ),
            (
                spawn,
                (selected_character.0.to_string(), selected_character.1.to_string(), selected_character.2.to_string())
            )
        )
    };
    let mut events = world.get_resource_mut::<Events<super::ChangeMap>>().unwrap();
    if map.0 != original_map.0 ||
       map.1 != original_map.1 ||
       map.2 != original_map.2 {
           events.send(super::ChangeMap {
               namespace: map.0,
               package: map.1,
               name: map.2
           });
       }
     if spawn_enemy {
         let mut events = world.get_resource_mut::<Events<crate::arena::SpawnEnemy>>().unwrap();
         events.send(crate::arena::SpawnEnemy {
             character: zombrr_core::packages::CharacterRef {
                 namespace: enemy.0,
                 package: enemy.1,
                 name: enemy.2
             },
             translation: Vec3::new(0.0, 4.0, 0.0)
         });
     }
}
