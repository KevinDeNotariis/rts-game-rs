use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use crate::components::units::{Health, IsMoving, Lifetime, Speed};
use crate::game_state::GameState;

use super::config::{UnitTypeConfig, UnitsConfig, UnitsConfigResource};

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Unit>()
            .register_type::<FastUnit>()
            .register_type::<TankUnit>()
            .add_systems(OnEnter(GameState::Playing), setup_enemy_factory)
            .add_systems(Update, move_unit.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
#[require(
    Mesh3d, 
    MeshMaterial3d<StandardMaterial>, 
    Health, 
    Lifetime,
    Speed,
)]
pub struct Unit(pub String);


#[derive(Reflect, Component, Default)]
#[reflect(Component)]
#[require(Unit)]
pub struct FastUnit;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
#[require(Unit)]
pub struct TankUnit;

#[derive(Reflect, Eq, PartialEq, Hash, Default)]
pub enum UnitType {
    #[default]
    FastUnit,
    TankUnit
}

pub struct UnitFactory {
    meshes: HashMap<UnitType, Capsule3d>,
    mesh_materials: HashMap<UnitType, Color>,
    config: UnitsConfig,
}

impl UnitFactory {
    pub fn new(config: UnitsConfig) -> Self {
        let mut meshes = HashMap::new();

        meshes.insert(UnitType::FastUnit, Capsule3d::new(0.1, 0.3));
        meshes.insert(UnitType::TankUnit, Capsule3d::new(0.5, 1.0));

        let mut mesh_materials = HashMap::new();

        mesh_materials.insert(UnitType::FastUnit, Color::srgb(0.2, 0.2, 0.0));
        mesh_materials.insert(UnitType::TankUnit, Color::srgb(0.7, 0.7, 0.7));
    
        Self {
            meshes,
            mesh_materials,
            config
        }
    }

    pub fn spawn(&self, 
        commands: &mut Commands, 
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        enemy_type: &UnitType, 
        position: &Vec3
    ) -> Entity {
        match enemy_type {
            UnitType::FastUnit => self.spawn_fast_unit(commands, meshes, materials, position),
            UnitType::TankUnit => self.spawn_tank_unit(commands, meshes, materials, position),
        }
    }

    pub fn spawn_fast_unit(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, position: &Vec3) -> Entity {
        let health = (self.config.base.health as f32 * self.config.units.get(&UnitTypeConfig::FastUnit).unwrap().health_m) as i32;
        let speed = self.config.base.speed as f32 * self.config.units.get(&UnitTypeConfig::FastUnit).unwrap().speed_m;

        commands.spawn(
            (
                FastUnit,
                Unit("Fast Unit".into()),
                Mesh3d(meshes.add(self.meshes.get(&UnitType::FastUnit).unwrap().clone())),
                MeshMaterial3d(materials.add(self.mesh_materials.get(&UnitType::FastUnit).unwrap().clone())),
                Transform::from_xyz(position.x, position.y, position.z),
                Health {
                    max: health,
                    current: health,
                },
                Lifetime(Timer::from_seconds(2., TimerMode::Once)),
                Speed(speed),
            )
        ).id()
    }

    pub fn spawn_tank_unit(&self, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>, position: &Vec3) -> Entity {
        let health = (self.config.base.health as f32 * self.config.units.get(&UnitTypeConfig::TankUnit).unwrap().health_m) as i32;
        let speed = self.config.base.speed as f32 * self.config.units.get(&UnitTypeConfig::TankUnit).unwrap().speed_m;

        commands.spawn(
            (
                TankUnit,
                Unit("Tank Unit".into()),
                Mesh3d(meshes.add(self.meshes.get(&UnitType::TankUnit).unwrap().clone())),
                MeshMaterial3d(materials.add(self.mesh_materials.get(&UnitType::TankUnit).unwrap().clone())),
                Transform::from_xyz(position.x, position.y, position.z),
                Health {
                    max: health,
                    current: health,
                },
                Lifetime(Timer::from_seconds(2., TimerMode::Once)),
                Speed(speed),
            )
        ).id()
    }
}

#[derive(Resource)]
pub struct UnitFactoryResource {
    pub factory: UnitFactory,
}

fn setup_enemy_factory(mut commands: Commands, config: Res<UnitsConfigResource>, units_config_asset: Res<Assets<UnitsConfig>>) {
    let config = units_config_asset.get(&config.units).expect("Error getting units config");
    
    let factory = UnitFactory::new(config.clone());

    commands.insert_resource(UnitFactoryResource { factory});
}

fn move_unit(mut unit_query: Query<(&Speed, &mut Transform), With<IsMoving>>, time: Res<Time>) {
    for (speed, mut transform) in &mut unit_query {
        transform.translation.x += speed.0 * time.delta_secs();
    }
}
