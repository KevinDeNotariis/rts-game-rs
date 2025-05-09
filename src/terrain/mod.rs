use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use serde::{Deserialize, Serialize};
use systems::TerrainSystemsPlugins;

use crate::{asset_loaders::GenericYamlAssetLoader, game_state::GameState};

pub mod systems;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainResource {
            cursor_projection: Vec2::ZERO,
        })
        .register_type::<TerrainConfig>()
        .init_resource::<TerrainConfigResource>()
        .init_asset::<TerrainConfig>()
        .init_asset_loader::<GenericYamlAssetLoader<TerrainConfig>>()
        .add_systems(OnEnter(GameState::Loading), asset_load)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_plugins(TerrainSystemsPlugins);
    }
}

fn asset_load(mut config: ResMut<TerrainConfigResource>, asset_server: Res<AssetServer>) {
    config.0 = asset_server.load("config/terrain.yaml");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain_config_resource: Res<TerrainConfigResource>,
    terrain_config: Res<Assets<TerrainConfig>>,
) {
    let terrain_dim = terrain_config
        .get(&terrain_config_resource.0)
        .unwrap()
        .dimensions;

    commands.spawn((
        Name::new("Terrain"),
        Terrain,
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, terrain_dim))),
        MeshMaterial3d(materials.add(Color::srgb_u8(111, 78, 55))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Collider::cuboid(terrain_dim.x, 0.01, terrain_dim.y),
    ));
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct TerrainResource {
    pub cursor_projection: Vec2,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Terrain;

#[derive(Reflect, Deserialize, Serialize, Asset, Clone, Default)]
pub struct TerrainConfig {
    pub dimensions: Vec2,
    pub tile_size: f32,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TerrainConfigResource(pub Handle<TerrainConfig>);
