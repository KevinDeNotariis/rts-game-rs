use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, render::ColliderDebugColor};

use crate::base::{buildings::components::BuildingGettingPlaced, factions::RomeBuildingType};

use super::assets::RomeBuildingsAssets;

pub struct RomeBuildingsFactoryPlugin;

impl Plugin for RomeBuildingsFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RomeBuildingsFactory>()
            .register_type::<RomeBuildingsFactoryResource>()
            .insert_resource(RomeBuildingsFactoryResource {
                factory: RomeBuildingsFactory {
                    config: RomeBuildingsConfig { size: 2.0 },
                },
            });
    }
}

#[derive(Reflect, Clone, Copy)]
pub struct RomeBuildingsConfig {
    size: f32,
}

#[derive(Reflect, Clone, Copy)]
pub struct RomeBuildingsFactory {
    pub config: RomeBuildingsConfig,
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RomeBuildingsFactoryResource {
    pub factory: RomeBuildingsFactory,
}

impl RomeBuildingsFactory {
    fn building(&self, building_type: RomeBuildingType, pos: Vec3) -> impl Bundle {
        match building_type {
            RomeBuildingType::Cottage => (
                Name::new("RomeCottage"),
                Transform {
                    translation: pos,
                    scale: Vec3::splat(self.config.size),
                    ..default()
                },
                RapierPickable,
                Visibility::Visible,
                Collider::cuboid(1.0 / 4., 1.0 / 4., 1.0 / 4.),
                ColliderDebugColor(Hsla::hsl(220.0, 1.0, 0.3)),
            ),
            RomeBuildingType::Barracks => todo!(),
            RomeBuildingType::Armory => todo!(),
        }
    }

    pub fn spawn_holo<'a>(
        &self,
        commands: &'a mut Commands,
        building_type: RomeBuildingType,
        pos: Vec3,
    ) -> EntityCommands<'a> {
        commands.spawn((BuildingGettingPlaced, self.building(building_type, pos)))
    }

    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        rome_building_assets: &RomeBuildingsAssets,
        assets_gltf: Res<Assets<Gltf>>,
        building_type: RomeBuildingType,
        pos: Vec3,
    ) -> EntityCommands<'a> {
        let model_gltf = rome_building_assets.model.get(&building_type).unwrap();
        let Some(model) = assets_gltf.get(model_gltf) else {
            panic!("Asset not loaded proprely");
        };

        commands.spawn((
            self.building(building_type, pos),
            SceneRoot(model.scenes[0].clone()),
        ))
    }
}
