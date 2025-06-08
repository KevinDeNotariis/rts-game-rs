use bevy::prelude::*;

use crate::game_states::GameState;

pub struct GameSelectionPlugin;

impl Plugin for GameSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameSelection), setup);
    }
}

fn setup(mut next_state_res: ResMut<NextState<GameState>>) {
    next_state_res.set(GameState::Playing);
}
