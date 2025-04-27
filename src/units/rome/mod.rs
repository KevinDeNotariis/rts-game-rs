use animations::{RomeUnitAnimationPlay, RomeUnitsAnimationAssets};
use bevy::{ecs::observer::TriggerTargets, prelude::*, scene::SceneInstanceReady};
use config::RomeUnitType;
use factory::RomeUnitsFactoryResource;

use crate::game_state::GameState;

pub mod animations;
pub mod archer;
pub mod config;
pub mod factory;
pub mod legionary;

pub struct RomeUnitsPlugin;

impl Plugin for RomeUnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            config::RomeUnitsConfigPlugin,
            factory::RomeUnitsFactoryPlugin,
            animations::RomeUnitsAnimationPlugin,
        ))
        .add_systems(OnEnter(GameState::Playing), spawn_units);
    }
}

fn spawn_units(
    mut commands: Commands,
    factory_resource: ResMut<RomeUnitsFactoryResource>,
    rome_animations: Res<RomeUnitsAnimationAssets>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    factory_resource.factory.spawn(
        &mut commands,
        &RomeUnitType::Legionary,
        &Vec3::new(0.0, 0.0, 0.0),
        &rome_animations,
        &mut graphs,
    );

    factory_resource.factory.spawn(
        &mut commands,
        &RomeUnitType::Archer,
        &Vec3::new(2.0, 0.0, 2.0),
        &rome_animations,
        &mut graphs,
    );
}
