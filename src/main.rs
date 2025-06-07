use anyhow::Error;
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{RapierPickingPlugin, RapierPickingSettings},
    render::RapierDebugRenderPlugin,
};
use rts_game_rs::game_states::GameStatePlugin;

fn main() -> Result<(), Error> {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "RTS".into(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            RapierPickingPlugin,
        ))
        .insert_resource(RapierPickingSettings {
            require_markers: true,
            ..default()
        })
        .add_plugins(GameStatePlugin)
        .run();

    Ok(())
}
