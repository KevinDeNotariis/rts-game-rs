use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub mod camera;
use camera::*;

use tower_defense_rs::{
    game_state::{GameState, GameStatePlugin},
    spawns::SpawnPointPlugin,
    start_menu::StartMenuPlugin,
    units::UnitsPlugin,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn Terrain
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(5.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

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
                    resolution: (800.0, 500.0).into(),
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(GameStatePlugin)
        .init_state::<GameState>()
        .add_systems(Startup, load_assets)
        // .add_loading_state(
        //     LoadingState::new(GameState::Loading).continue_to_state(GameState::StartMenu),
        // )
        .add_plugins(CameraPlugin)
        .add_plugins(StartMenuPlugin)
        .add_plugins(SpawnPointPlugin)
        .add_plugins(UnitsPlugin)
        .add_systems(
            OnEnter(GameState::Playing),
            setup.run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn load_assets(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::StartMenu);
}
