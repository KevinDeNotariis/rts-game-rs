use bevy::{platform::collections::HashMap, prelude::*};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{
        LoadingStateAppExt,
        config::{ConfigureLoadingState, LoadingStateConfig},
    },
};

use crate::{base::factions::RomeBuildingType, game_state::GameState};

pub struct RomeBuildingsAssetsPlugin;

impl Plugin for RomeBuildingsAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RomeBuildingsAssets>()
            .configure_loading_state(
                LoadingStateConfig::new(GameState::Loading)
                    .load_collection::<RomeBuildingsAssets>(),
            );
    }
}

#[derive(Resource, AssetCollection, Clone, Reflect)]
#[reflect(Resource)]
pub struct RomeBuildingsAssets {
    #[asset(paths("buildings/Cottage.glb"), collection(typed, mapped))]
    pub model: HashMap<RomeBuildingType, Handle<Gltf>>,
}
