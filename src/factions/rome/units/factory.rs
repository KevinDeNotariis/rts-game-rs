use bevy::prelude::*;

use crate::{
    components::units::{IsMoving, Lifetime, Speed},
    game_state::GameState,
};

use super::{
    animations::{RomeUnitAnimationPlay, RomeUnitsAnimationAssets, play_animation_when_ready},
    archer::RomeArcher,
    config::RomeUnitType,
    legionary::RomeLegionary,
};

pub struct RomeUnitsFactoryPlugin;

impl Plugin for RomeUnitsFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<RomeUnitsFactory>()
            .register_type::<RomeUnitsFactoryResource>()
            .add_systems(Startup, setup)
            .add_systems(Update, movement.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Reflect, Clone, Copy)]
pub struct RomeUnitsFactory;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct RomeUnitsFactoryResource {
    pub factory: RomeUnitsFactory,
}

impl RomeUnitsFactory {
    pub fn spawn(
        &self,
        commands: &mut Commands,
        unit_type: &RomeUnitType,
        pos: &Vec3,
        rome_animations: &RomeUnitsAnimationAssets,
        graphs: &mut ResMut<Assets<AnimationGraph>>,
    ) -> Entity {
        let (graph, index) = AnimationGraph::from_clip(rome_animations.run_animation.clone());

        let graph_handle = graphs.add(graph);

        let unit_animation_play = RomeUnitAnimationPlay {
            graph_handle,
            index,
        };

        match unit_type {
            RomeUnitType::Legionary => commands
                .spawn((
                    RomeLegionary,
                    Transform::from_translation(*pos),
                    unit_animation_play,
                    SceneRoot(rome_animations.scene.clone()),
                    IsMoving,
                    Lifetime(Timer::from_seconds(10.0, TimerMode::Once)),
                ))
                .observe(play_animation_when_ready)
                .id(),
            RomeUnitType::Archer => commands
                .spawn((
                    RomeArcher,
                    Transform::from_translation(*pos),
                    unit_animation_play,
                    SceneRoot(rome_animations.scene.clone()),
                    IsMoving,
                    Lifetime(Timer::from_seconds(10.0, TimerMode::Once)),
                ))
                .observe(play_animation_when_ready)
                .id(),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(RomeUnitsFactoryResource {
        factory: RomeUnitsFactory,
    });
}

fn movement(mut units: Query<(&mut Transform, &Speed, &IsMoving)>, time: Res<Time>) {
    for (mut transform, speed, _) in &mut units {
        transform.translation += Vec3::X * speed.0 * time.delta_secs();
    }
}
