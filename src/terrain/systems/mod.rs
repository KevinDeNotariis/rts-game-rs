use bevy::prelude::*;

use crate::game_state::GameState;

pub mod raycast;

pub struct TerrainSystemsPlugins;

impl Plugin for TerrainSystemsPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            raycast::raycast_cursor_to_terrain.run_if(in_state(GameState::Playing)),
        );
    }
}
