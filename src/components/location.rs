use bevy::prelude::*;

use crate::units::rome::config::RomeUnitType;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SpawnLocation {
    pub spawn_timer: Timer,
    pub unit_type: RomeUnitType,
    pub position: Vec3,
}
