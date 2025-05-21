use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildingGettingPlaced;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct BuildingSelected;
