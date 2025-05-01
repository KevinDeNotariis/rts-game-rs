use bevy::prelude::*;

pub mod ui;

pub struct RomeBuildingsPlugin;

impl Plugin for RomeBuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ui::RomeBuildingsUIPlugin);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RomeBuildingUnitsSpawn;
