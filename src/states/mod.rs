use bevy::prelude::*;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum UserActionState {
    #[default]
    None,
    PlacingBuilding,
    BuildingSelected,
    UnitsSelected,
}
