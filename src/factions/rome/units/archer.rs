use crate::base::{
    abilities::AbilityTripleDamage,
    units::{AttackRange, Health, Speed},
};
use bevy::prelude::*;
use spawnable_unit_derive::SpawnableUnit;

pub trait SpawnableUnit {
    fn build_unit(&self) -> &'static str;
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[derive(SpawnableUnit)]
#[spawnable_unit(unit_type("Archer"))]
#[require(
    Name::new("Archer"),
    Transform,
    Health{max: 50, current: 50},
    Speed(1.0),
    AttackRange(20),
    AbilityTripleDamage
)]
pub struct RomeArcher;
