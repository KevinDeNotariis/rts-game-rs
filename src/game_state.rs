use bevy::{dev_tools::states::log_transitions, prelude::*};

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (log_transitions::<GameState>, exit_game));
    }
}

fn exit_game(key: Res<ButtonInput<KeyCode>>, mut app_exit_event: EventWriter<AppExit>) {
    if key.pressed(KeyCode::SuperLeft) && key.pressed(KeyCode::Escape) {
        app_exit_event.write(AppExit::Success);
    }
}
