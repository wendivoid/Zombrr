use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use zombrr_core::{ArenaState, ZombrrState};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<super::FpsText>()
            .register_type::<super::DisplayRoot>()
            .register_type::<super::TextField>()
            .register_type::<super::KillCountText>()
            // NOTE: This is required to use Text Bundle in scenes ?
            .register_type::<Option<f32>>()
            .add_system(update_fps_text.system())
            .add_system(update_killcount_text.system())
            .add_system(super::add_focus_policy.system())
            .add_system_set(
                SystemSet::on_enter(ZombrrState::Arena(ArenaState::Loading))
                    .with_system(super::init::initialize_user_interface.system()),
            )
            .add_system_set(
                SystemSet::on_update(ZombrrState::Arena(ArenaState::Loading)).with_system(
                    bevy_loading::track(super::init::check_environment_scene.system()),
                ),
            );
    }
}

fn update_fps_text(
    mut commands: Commands,
    assets: Res<AssetServer>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<(Entity, Option<&mut Text>), With<super::FpsText>>,
) {
    for (entity, ref mut text) in query.iter_mut() {
        let mut fps = 0.0;
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                fps = fps_avg;
            }
        }
        if let Some(text) = text {
            if text.sections.is_empty() {
                text.sections.push(TextSection {
                    value: format!("FPS: {:.0}", fps),
                    ..Default::default()
                });
            } else {
                if let Some(mut text) = text.sections.get_mut(0) {
                    text.value = format!("FPS: {:.0}", fps);
                }
            }
        } else {
            commands.entity(entity).insert(Text {
                sections: vec![TextSection {
                    value: format!("FPS: {:.0}", fps),
                    style: TextStyle {
                        font: assets.load("packages/zombrr/debug/fonts/VictorMono.ttf"),
                        font_size: 30.0,
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            });
        }
    }
}

fn update_killcount_text(
    mut commands: Commands,
    assets: Res<AssetServer>,
    killcounts: Res<zombrr_damage::KillCount>,
    mut query: Query<(Entity, Option<&mut Text>), With<super::KillCountText>>,
) {
    for (entity, ref mut text) in query.iter_mut() {
        let string = killcounts
            .0
            .iter()
            .map(|(key, value)| format!("Entity({:?}) -> {}", key, value))
            .collect::<Vec<String>>();
        if let Some(text) = text {
            if text.sections.is_empty() {
                text.sections.push(TextSection {
                    value: string.join("\n"),
                    ..Default::default()
                });
            } else {
                if let Some(mut text) = text.sections.get_mut(0) {
                    text.value = string.join("\n");
                }
            }
        } else {
            commands.entity(entity).insert(Text {
                sections: vec![TextSection {
                    value: string.join("\n"),
                    style: TextStyle {
                        font: assets.load("packages/zombrr/debug/fonts/VictorMono.ttf"),
                        font_size: 30.0,
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                ..Default::default()
            });
        }
    }
}
