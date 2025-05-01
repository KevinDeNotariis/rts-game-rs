use bevy::prelude::*;

use crate::components::{
    abilities::AbilityTripleDamage,
    units::{AttackRange, Health, Speed},
};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(
    Name::new("Archer"),
    Transform,
    Health{max: 50, current: 50},
    Speed(1.0),
    AttackRange(20),
    AbilityTripleDamage
)]
pub struct RomeArcher;
