use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub max: i32,
    pub current: i32,
}

// Marker component to associate to units that are currently moving
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IsMoving;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime(pub Timer);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct AttackRange(pub i32);

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub enum UnitState {
    #[default]
    Idle,
    Walking,
    Fighting,
    Dead,
}
