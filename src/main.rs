use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub mod camera;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::{RapierPickingPlugin, RapierPickingSettings},
    render::RapierDebugRenderPlugin,
};
use camera::*;

use tower_defense_rs::{
    factions::FactionsPlugin,
    game_state::{GameState, GameStatePlugin},
    start_menu::StartMenuPlugin,
    terrain::TerrainPlugin,
};

fn setup(mut commands: Commands) {
    // Spawn Light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "My Game".into(),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin::default(),
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
            // MeshPickingPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            RapierPickingPlugin,
        ))
        .insert_resource(RapierPickingSettings {
            require_markers: true,
            ..default()
        })
        .add_plugins(GameStatePlugin)
        .init_state::<GameState>()
        .add_systems(Startup, load_assets)
        .add_plugins((StartMenuPlugin, CameraPlugin, TerrainPlugin, FactionsPlugin))
        .add_systems(
            OnEnter(GameState::Playing),
            setup.run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn load_assets(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::StartMenu);
}
