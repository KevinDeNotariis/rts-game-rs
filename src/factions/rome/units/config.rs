use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset_loaders::GenericYamlAssetLoader;

pub struct RomeUnitsConfigPlugin;

impl Plugin for RomeUnitsConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RomeUnitsConfig>()
            .init_resource::<RomeUnitsConfigResource>()
            .init_asset::<RomeUnitsConfig>()
            .init_asset_loader::<GenericYamlAssetLoader<RomeUnitsConfig>>()
            .add_systems(Startup, setup);
    }
}

#[derive(Reflect, Default)]
pub enum RomeUnitType {
    #[default]
    Legionary,
    Archer,
}

#[derive(Reflect, Deserialize, Serialize, Debug, Clone, Default)]
pub struct RomeLegionaryUnitConfig {
    pub health: i32,
    pub speed: f32,
}

#[derive(Reflect, Deserialize, Serialize, Debug, Clone, Default)]
pub struct RomeArcherUnitConfig {
    pub health: i32,
    pub speed: f32,
}

#[derive(Reflect, Deserialize, Serialize, Asset, Clone, Default)]
pub struct RomeUnitsConfig {
    pub legionary: RomeLegionaryUnitConfig,
    pub archer: RomeArcherUnitConfig,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct RomeUnitsConfigResource(Handle<RomeUnitsConfig>);

fn setup(mut config: ResMut<RomeUnitsConfigResource>, asset_server: Res<AssetServer>) {
    config.0 = asset_server.load("config/units/rome.yaml");
}
