use bevy::prelude::*;

pub mod buildings;
pub mod units;

pub struct RomePlugin;

impl Plugin for RomePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((units::RomeUnitsPlugin, buildings::RomeBuildingsPlugin));
    }
}
