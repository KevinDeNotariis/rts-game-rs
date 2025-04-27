use bevy::prelude::*;

use crate::components::{
    abilities::AbilityImmunity,
    units::{Health, Name, Speed},
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name("Legionary".into()),
    Health{max: 100, current: 100},
    Speed(0.5),
    AbilityImmunity
)]
pub struct RomeLegionary;
