use bevy::{platform::collections::HashMap, prelude::*};

use crate::game_state::GameState;

use super::models::RomeBuildingType;

pub struct RomeBuildingsAssetsPlugin;

impl Plugin for RomeBuildingsAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RomeBuildingsAssets>()
            .add_systems(Startup, setup.run_if(in_state(GameState::Loading)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) -> Result {
    let scene: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("buildings/Cottage.glb"));

    commands.insert_resource(RomeBuildingsAssets {
        model: HashMap::from_iter([(RomeBuildingType::Cottage, scene)]),
    });

    Ok(())
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct RomeBuildingsAssets {
    pub model: HashMap<RomeBuildingType, Handle<Scene>>,
}
