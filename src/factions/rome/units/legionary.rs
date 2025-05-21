use bevy::prelude::*;

use crate::base::{
    abilities::AbilityImmunity,
    units::{Health, Speed},
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name::new("Legionary"),
    Health{max: 100, current: 100},
    Speed(0.5),
    AbilityImmunity
)]
pub struct RomeLegionary;
