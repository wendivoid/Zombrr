use bevy::prelude::*;
use bevy_devtools::{ Tool, egui, Settings };

use zombrr::arena::modes::Mode;

pub fn change_mode() -> Tool {
    Tool {
        name: "change-mode".into(),
        label: Some("Change Mode".into()),
        priority: 1,
        perform_icon: None,
        perform: None,
        render: render_change_mode
    }
}

fn render_change_mode(ui: &mut egui::Ui, _: &mut Settings, world: &mut World) {
    let mut options = world.get_resource_mut::<zombrr::arena::modes::Mode>().unwrap();
    ui.horizontal(|ui| {
        egui::ComboBox::from_label("Change Mode")
        .selected_text(&format!("{:?}", *options))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut *options, Mode::None, "None");
            ui.selectable_value(&mut *options, Mode::OneEnemy, "One Enemy");
        });
    });
}

pub fn handle_change_mode(
    mut res: ResMut<Mode>,
    mut events: EventReader<crate::events::ChangeMode>,
) {
    for event in events.iter() {

        *res = match event.name.as_str() {
            "mode" => Mode::None,
            "OneEnemy" => Mode::OneEnemy,
            other => Mode::Custom(other.into())
        };
    }
}
