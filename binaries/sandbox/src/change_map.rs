use crate::events::ChangeMap;
use bevy::prelude::*;
use bevy_devtools::{egui, Settings, Tool};
use zombrr_core::{ArenaOptions, MenuState, ZombrrPackages, ZombrrState};

pub fn change_map() -> Tool {
    Tool {
        name: "change-map".into(),
        label: Some("Change Map".into()),
        priority: 1,
        perform_icon: None,
        perform: None,
        render: render_change_map,
    }
}

fn render_change_map(ui: &mut egui::Ui, _: &mut Settings, world: &mut World) {
    let (selected, original) = {
        let options = world.get_resource::<ArenaOptions>().unwrap();
        let packages = world.get_resource::<ZombrrPackages>().unwrap();
        let original_map = (
            options.map.package.as_str(),
            options.map.package.as_str(),
            options.map.name.as_str(),
        );
        let mut selected_map = (
            options.map.package.as_str(),
            options.map.package.as_str(),
            options.map.name.as_str(),
        );
        ui.horizontal(|ui| {
            egui::ComboBox::from_label("Change Map")
                .selected_text(&options.map.name)
                .show_ui(ui, |ui| {
                    for (namespace, packages) in packages.get_maps() {
                        for (package, maps) in packages {
                            for map in maps {
                                ui.selectable_value(
                                    &mut selected_map,
                                    (namespace, package, &map.name),
                                    &map.name,
                                );
                            }
                        }
                    }
                });
        });
        (
            (
                selected_map.0.to_string(),
                selected_map.1.to_string(),
                selected_map.2.to_string(),
            ),
            (
                original_map.0.to_string(),
                original_map.1.to_string(),
                original_map.2.to_string(),
            ),
        )
    };
    let mut events = world
        .get_resource_mut::<bevy::app::Events<ChangeMap>>()
        .unwrap();
    if selected.0 != original.0 || selected.1 != original.1 || selected.2 != original.2 {
        events.send(ChangeMap {
            namespace: selected.0.into(),
            package: selected.1.into(),
            name: selected.2.into(),
        });
    }
}

pub fn handle_change_map(
    mut events: EventReader<crate::events::ChangeMap>,
    mut options: ResMut<ArenaOptions>,
    mut state: ResMut<State<ZombrrState>>,
) {
    for event in events.iter() {
        options.map.namespace = event.namespace.clone();
        options.map.package = event.package.clone();
        options.map.name = event.name.clone();
        state.set(ZombrrState::Menu(MenuState::Loading)).unwrap();
    }
}
