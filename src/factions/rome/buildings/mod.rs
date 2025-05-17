use bevy::prelude::*;

pub mod assets;
pub mod factory;
pub mod lifecycle;
pub mod models;
pub mod ui;

pub struct RomeBuildingsPlugin;

impl Plugin for RomeBuildingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ui::RomeBuildingsUIPlugin,
            factory::RomeBuildingsFactoryPlugin,
            assets::RomeBuildingsAssetsPlugin,
            lifecycle::RomeBuildingsLifecyclePlugin,
        ));
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RomeBuildingUnitsSpawn;
