use bevy::prelude::*;

use crate::units::base::UnitType;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct SpawnLocation {
    pub spawn_timer: Timer,
    pub unit_type: UnitType,
    pub position: Vec3,
}
