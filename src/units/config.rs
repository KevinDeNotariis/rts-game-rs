use std::collections::HashMap;

use crate::asset_loaders::GenericYamlAssetLoader;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub struct ConfigAssetPlugin;

impl Plugin for ConfigAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitsConfigResource>()
            .init_asset::<UnitsConfig>()
            .init_asset_loader::<GenericYamlAssetLoader<UnitsConfig>>()
            .add_systems(Startup, setup);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BaseUnitConfig {
    pub health: i32,
    pub speed: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitConfig {
    pub health_m: f32,
    pub speed_m: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
pub enum UnitTypeConfig {
    #[serde(rename = "fast_unit")]
    FastUnit,
    #[serde(rename = "tank_unit")]
    TankUnit,
}

#[derive(Debug, Serialize, Deserialize, Clone, Asset, TypePath, Default)]
pub struct UnitsConfig {
    pub base: BaseUnitConfig,
    pub units: HashMap<UnitTypeConfig, UnitConfig>,
}

#[derive(Resource, Default)]
pub struct UnitsConfigResource {
    pub units: Handle<UnitsConfig>,
}

fn setup(mut config: ResMut<UnitsConfigResource>, asset_server: Res<AssetServer>) {
    config.units = asset_server.load("config/units.yaml");
}
