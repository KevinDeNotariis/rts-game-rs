use animations::UnitAnimations;
use bevy::prelude::*;

mod animations;
mod assets;
pub mod base;
pub mod config;

pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UnitAnimations>()
            .add_plugins((config::ConfigAssetPlugin, base::UnitPlugin));
    }
}
