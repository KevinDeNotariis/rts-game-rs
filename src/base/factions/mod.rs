use core::panic;

use bevy::prelude::*;
use bevy_asset_loader::mapped::MapKey;

#[derive(Reflect, Debug, Hash, Eq, PartialEq, Clone)]
pub enum Factions {
    Rome,
}

#[derive(Reflect, Eq, PartialEq, Hash, Clone)]
pub enum RomeBuildingType {
    Cottage,
    Barracks,
    Armory,
}

impl MapKey for RomeBuildingType {
    fn from_asset_path(path: &bevy::asset::AssetPath) -> Self {
        let path = path.path();

        if path == std::path::Path::new("buildings/Cottage.glb") {
            return Self::Cottage;
        } else if path == std::path::Path::new("buildings/Armory.glb") {
            return Self::Armory;
        } else if path == std::path::Path::new("buildings/Barracks.glb") {
            return Self::Barracks;
        }

        panic!("Should be here");
    }
}
