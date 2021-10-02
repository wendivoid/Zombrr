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
    let ((namespace, package, map), original) = {
        let packages = world.get_resource::<ZombrrPackages>().unwrap();
        let options = world.get_resource::<ArenaOptions>().unwrap();
        let original_value = (options.map.package.as_str(), options.map.package.as_str(), options.map.name.as_str());
        let mut selected_value = (options.map.package.as_str(), options.map.package.as_str(), options.map.name.as_str());
        ui.group(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Change Map")
                .selected_text(&options.map.name)
                .show_ui(ui, |ui| {
                    for (namespace, packages) in packages.get_maps() {
                        for (package, maps) in packages {
                            for map in maps {
                                ui.selectable_value(&mut selected_value, (namespace, package, &map.name), &map.name);
                            }
                        }
                    }
                });
            });
        });
        (
            (selected_value.0.to_string(), selected_value.1.to_string(), selected_value.2.to_string()),
            (original_value.0.to_string(), original_value.1.to_string(), selected_value.2.to_string())
        )
    };
    let mut events = world.get_resource_mut::<Events<super::ChangeMap>>().unwrap();
    if namespace != original.0 ||
       package != original.1 ||
       map != original.2 {
           events.send(super::ChangeMap {
               namespace,
               package,
               name: map
           });
       }

}
