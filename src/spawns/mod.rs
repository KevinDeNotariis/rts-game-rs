// use bevy::prelude::*;

// use crate::components::{location::SpawnLocation, units::Lifetime};
// use crate::game_state::GameState;
// use crate::units::rome::animations::RomeUnitsAnimationAssets;
// use crate::units::rome::config::RomeUnitType;
// use crate::units::rome::factory::RomeUnitsFactoryResource;

// pub struct SpawnPointPlugin;

// impl Plugin for SpawnPointPlugin {
//     fn build(&self, app: &mut App) {
//         app.register_type::<SpawnLocation>()
//             .add_systems(OnEnter(GameState::Playing), setup)
//             .add_systems(
//                 Update,
//                 (spawn, despawn).run_if(in_state(GameState::Playing)),
//             );
//     }
// }

// fn setup(mut commands: Commands) {
//     // Spawn SpawnLocation for Fast Units
//     commands.spawn(SpawnLocation {
//         spawn_timer: Timer::from_seconds(3., TimerMode::Repeating),
//         unit_type: RomeUnitType::Archer,
//         position: Vec3 {
//             x: 0.0,
//             y: 0.0,
//             z: 0.0,
//         },
//     });

//     // Spawn SpawnLocation for Tank Units
//     commands.spawn(SpawnLocation {
//         spawn_timer: Timer::from_seconds(3., TimerMode::Repeating),
//         unit_type: RomeUnitType::Legionary,
//         position: Vec3 {
//             x: 2.0,
//             y: 0.0,
//             z: 2.0,
//         },
//     });
// }

// #[derive(Resource)]
// struct Spawned {
//     spawned_entity: Entity,
// }

// fn spawn(
//     mut commands: Commands,
//     mut spawn_location_query: Query<&mut SpawnLocation>,
//     unit_factory: ResMut<RomeUnitsFactoryResource>,
//     rome_animations: Res<RomeUnitsAnimationAssets>,
//     mut graphs: ResMut<Assets<AnimationGraph>>,
//     time: Res<Time>,
// ) {
//     for mut spawn_location in &mut spawn_location_query {
//         spawn_location.spawn_timer.tick(time.delta());

//         if spawn_location.spawn_timer.just_finished() {
//             let spawned = unit_factory.factory.spawn(
//                 &mut commands,
//                 &spawn_location.unit_type,
//                 &spawn_location.position,
//                 &rome_animations,
//                 &mut graphs,
//             );

//             commands.insert_resource(Spawned {
//                 spawned_entity: spawned,
//             });
//         }
//     }
// }

// fn despawn(
//     mut commands: Commands,
//     mut lifetime_query: Query<(Entity, &mut Lifetime)>,
//     time: Res<Time>,
// ) {
//     for (entity, mut lifetime) in &mut lifetime_query {
//         lifetime.0.tick(time.delta());

//         if lifetime.0.just_finished() {
//             commands.entity(entity).despawn();
//         }
//     }
// }
