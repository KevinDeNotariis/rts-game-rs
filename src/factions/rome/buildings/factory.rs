use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, render::ColliderDebugColor};

use super::{assets::RomeBuildingsAssets, ui::BuildingGettingPlaced};

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

#[derive(Reflect, Eq, PartialEq, Hash, Clone)]
pub enum RomeBuildingType {
    Cottage,
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
        }
    }

    pub fn spawn_holo<'a>(
        &self,
        commands: &'a mut Commands,
        building_type: RomeBuildingType,
        pos: Vec3,
    ) -> EntityCommands<'a> {
        match building_type {
            RomeBuildingType::Cottage => {
                commands.spawn((BuildingGettingPlaced, self.building(building_type, pos)))
            }
        }
    }

    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        rome_building_assets: &RomeBuildingsAssets,
        building_type: RomeBuildingType,
        pos: Vec3,
    ) -> EntityCommands<'a> {
        let model = rome_building_assets.model.get(&building_type).unwrap();

        match building_type {
            RomeBuildingType::Cottage => {
                commands.spawn((self.building(building_type, pos), SceneRoot(model.clone())))
            }
        }
    }
}
