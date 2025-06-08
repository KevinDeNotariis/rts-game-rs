use bevy::{dev_tools::states::log_transitions, prelude::*};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (log_transitions::<GameState>, exit_game));
    }
}

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    GameSelection,
    LoadGame,
    Playing,
    MapEditor,
    HeroEditor,
    Options,
    Credits,
    ExitingGame,
}

fn exit_game(key: Res<ButtonInput<KeyCode>>, mut app_exit_event: EventWriter<AppExit>) {
    if key.pressed(KeyCode::SuperLeft) && key.pressed(KeyCode::Escape) {
        app_exit_event.write(AppExit::Success);
    }
}
